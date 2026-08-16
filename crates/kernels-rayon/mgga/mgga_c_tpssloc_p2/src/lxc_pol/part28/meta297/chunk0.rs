//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1208/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1208(t340: f64, t63: f64, t344: f64, t221: f64, t339: f64, t2960: f64, t2974: f64, t135: f64, t3016: f64, t973: f64, t1036: f64, t3078: f64) -> (f64, f64, f64, f64, f64) {
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = 0.3086419753086419753e-3_f64 * t339 * t10337;
    let t10342 = t2960 * t2974;
    let t10352 = t135 * t3016;
    let t10353 = t973 * t10352;
    let t10370 = t3078 * t1036;
    (t10335, t10339, t10342, t10353, t10370)
}
