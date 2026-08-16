//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk755;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta148(t2932: f64, t950: f64, t2978: f64, t60: f64, t344: f64, t2987: f64, t340: f64, t974: f64, t247: f64, t375: f64, t1043: f64, t2775: f64, t2770: f64, t3061: f64, t1022: f64, t3131: f64, t3188: f64, t1932: f64, t360: f64, t193: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4497, t4509, t4510, t4518, t4546, t4582) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk755(t2932, t950, t2978, t60, t344, t2987, t340, t974, t247, t375);
        let (t4583, t4588, t4594, t4673, t4684, t4700) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk756(t1043, t2775, t2770, t3061, t1022, t3131, t3188, t1932, t360, t193, t336);
    (t4497, t4509, t4510, t4518, t4546, t4582, t4583, t4588, t4594, t4673, t4684, t4700)
}
