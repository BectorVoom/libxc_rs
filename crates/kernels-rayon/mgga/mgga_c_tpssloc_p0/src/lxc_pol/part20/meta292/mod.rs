//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1501;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1502;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta292(t10277: f64, t2978: f64, t9288: f64, t974: f64, t1030: f64, t363: f64, t3068: f64, t1058: f64, t10213: f64, t10216: f64, t3030: f64, t990: f64, t3032: f64, t3129: f64, t3038: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10931, t10932, t10935, t10936, t10937) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1501(t10277, t2978, t9288, t974, t1030, t363, t3068, t1058);
        let (t10943, t10944, t10947, t10948, t10949) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1502(t10213, t10216, t9288, t974, t3030, t990, t3032, t3129);
        let t10952 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1503(t10948, t3038);
    (t10931, t10932, t10935, t10936, t10937, t10943, t10944, t10947, t10948, t10949, t10952)
}
