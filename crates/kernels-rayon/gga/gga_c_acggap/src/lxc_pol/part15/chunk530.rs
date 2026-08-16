//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 530/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk530(t1172: f64, t1530: f64, t301: f64, t944: f64, t396: f64, t980: f64, t409: f64, t360: f64, t372: f64, t177: f64, t414: f64, t377: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3462 = t1530 * t1172;
    let t3463 = t944 * t301;
    let t3476 = t980 * t396;
    let t3477 = t3476 * t409;
    let t3539 = t944 * t360;
    let t3544 = t944 * t372;
    let t3551 = 0.30011812682648815881e-2_f64 * t980 * t414 * t177;
    let t3556 = 0.17006693853500995666e-1_f64 * t377 * t973 * t177;
    (t3462, t3463, t3476, t3477, t3539, t3544, t3551, t3556)
}
