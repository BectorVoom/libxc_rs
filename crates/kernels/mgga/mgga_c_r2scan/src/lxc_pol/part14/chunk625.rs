//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 625/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk625<F: Float>(t1053: F, t1102: F, t3692: F, t3465: F, t3574: F, t3262: F, t3469: F, t3579: F, t3275: F, t3472: F, t3582: F, t3476: F, t3477: F, t3478: F, t3485: F, t3486: F, t3491: F, t3586: F, t3589: F, t3592: F, t3595: F, t3598: F, t3600: F, t3604: F, t3608: F, t3611: F, t3615: F) -> (F, F, F, F, F, F) {
    let t3694 = t1102 * t1053 * t3692;
    let t3700 = t3465 * t3574;
    let t3701 = t3262 * t3700;
    let t3702 = 3.0 / 4.0 * t3701;
    let t3703 = t3579 * t3469;
    let t3704 = t3703 / 4.0;
    let t3706 = t3275 * t3472 * t3582;
    let t3707 = 5.0 / 16.0 * t3706;
    let t3718 = -t3476 + t3477 - t3478 - 0.10975748638225852664e0 * t3586 - 0.54878743191129263322e-1 * t3589 - 0.86682217400542685632e-1 * t3592 - 0.2600466522016280569e0 * t3595 - 0.86682217400542685632e-1 * t3598 + 0.86682217400542685632e-1 * t3600 - t3485 + t3486 + 0.43663693315433241794e-2 * t3604 + 0.13099107994629972538e-1 * t3608 + 0.43663693315433241794e-2 * t3611 - 0.43663693315433241794e-2 * t3615 - t3491;
    (t3694, t3700, t3702, t3704, t3707, t3718)
}
