//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 648/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk648(t1053: f64, t1102: f64, t3692: f64, t3465: f64, t3574: f64, t3262: f64, t3469: f64, t3579: f64, t3275: f64, t3472: f64, t3582: f64, t3476: f64, t3477: f64, t3478: f64, t3485: f64, t3486: f64, t3491: f64, t3586: f64, t3589: f64, t3592: f64, t3595: f64, t3598: f64, t3600: f64, t3604: f64, t3608: f64, t3611: f64, t3615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3694 = t1102 * t1053 * t3692;
    let t3700 = t3465 * t3574;
    let t3701 = t3262 * t3700;
    let t3702 = 3.0_f64 / 4.0_f64 * t3701;
    let t3703 = t3579 * t3469;
    let t3704 = t3703 / 4.0_f64;
    let t3706 = t3275 * t3472 * t3582;
    let t3707 = 5.0_f64 / 16.0_f64 * t3706;
    let t3718 = -t3476 + t3477 - t3478 - 0.10975748638225852664e0_f64 * t3586 - 0.54878743191129263322e-1_f64 * t3589 - 0.86682217400542685632e-1_f64 * t3592 - 0.2600466522016280569e0_f64 * t3595 - 0.86682217400542685632e-1_f64 * t3598 + 0.86682217400542685632e-1_f64 * t3600 - t3485 + t3486 + 0.43663693315433241794e-2_f64 * t3604 + 0.13099107994629972538e-1_f64 * t3608 + 0.43663693315433241794e-2_f64 * t3611 - 0.43663693315433241794e-2_f64 * t3615 - t3491;
    (t3694, t3700, t3702, t3704, t3707, t3718)
}
