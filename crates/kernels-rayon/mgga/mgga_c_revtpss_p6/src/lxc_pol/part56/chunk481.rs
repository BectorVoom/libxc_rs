//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 481/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk481(t3147: f64, t479: f64, t471: f64, t3153: f64, t1121: f64, t414: f64, t66: f64, t474: f64, t3089: f64, t1285: f64, t1264: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3598 = t479 * t3147;
    let t3603 = t471 * t471;
    let t3604 = t3153 * t3603;
    let t3617 = 1.0_f64 / t414 / t1121;
    let t3618 = t66 * t3617;
    let t3623 = t474 * t479;
    let t3624 = t3623 * t3089;
    let t3625 = t1285 * t3624;
    let t3626 = t828 * t1264;
    (t3598, t3603, t3604, t3617, t3618, t3623, t3624, t3625, t3626)
}
