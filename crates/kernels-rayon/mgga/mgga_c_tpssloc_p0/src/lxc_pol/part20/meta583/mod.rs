//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2152;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta583(t1058: f64, t3068: f64, t3087: f64, t363: f64, t11065: f64, t42387: f64, t10250: f64, t2970: f64, t973: f64, t10195: f64, t10231: f64, t1005: f64, t10375: f64, t10475: f64, t42342: f64, t42345: f64, t2770: f64, t283: f64, t10309: f64, t1041: f64, t10457: f64, t248: f64, t10444: f64, t354: f64, t364: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43358, t43361, t43374, t43377, t43382) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2152(t1058, t3068, t3087, t363, t11065, t42387, t10250, t2970, t973, t10195, t10231, t1005, t10375);
        let (t43385, t43398, t43406, t43410) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2153(t10475, t42342, t42345, t2770, t283, t10309, t1041, t10457, t248, t10444, t354, t364, t372);
    (t43358, t43361, t43374, t43377, t43382, t43385, t43398, t43406, t43410)
}
