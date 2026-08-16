//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk533;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta94(t1097: f64, t419: f64, t409: f64, t407: f64, t410: f64, t3236: f64, t281: f64, t2820: f64, t415: f64, t1176: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3262, t3263, t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3297) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk533(t1097, t419, t409, t407, t410, t3236, t281, t2820, t415, t1176, t241);
        let t3311 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk534(t1097);
    (t3262, t3263, t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3297, t3311)
}
