//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1171/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1171(t18330: f64, t18343: f64, t18361: f64, t18405: f64, t18454: f64, t18489: f64, t18524: f64, t18654: f64, t225: f64, t6048: f64, t886: f64, t11008: f64) -> (f64, f64, f64) {
    let t18657 = t18330 + t18343 + t18361 + t18405 + t18454 + t18489 + t18524 + t18654;
    let t18658 = t18657 * t225;
    let t18662 = t6048 * t886;
    let t18663 = t11008 * t18662;
    (t18657, t18658, t18663)
}
