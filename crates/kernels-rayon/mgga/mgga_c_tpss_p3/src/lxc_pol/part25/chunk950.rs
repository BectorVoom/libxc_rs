//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 950/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk950(t11942: f64, t3001: f64, t4180: f64, t11875: f64, t1505: f64, t2861: f64, t1053: f64, t4117: f64, t1523: f64, t2954: f64, t926: f64, t9637: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12146 = 0.11415555555555555555e-1_f64 * t11942;
    let t12210 = t4180 * t3001;
    let t12231 = 0.23744444444444444444e-1_f64 * t11875;
    let t12232 = 0.11872222222222222222e-1_f64 * t11942;
    let t12244 = t1505 * t2861;
    let t12264 = t4117 * t1053;
    let t12269 = t1523 * t2954;
    let t12278 = t926 * t9637;
    (t12146, t12210, t12231, t12232, t12244, t12264, t12269, t12278)
}
