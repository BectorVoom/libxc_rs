//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 652/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk652(t322: f64, t352: f64, t3549: f64, t3556: f64, t3675: f64, t3741: f64, t3743: f64, t3771: f64, t3774: f64, t855: f64, t3564: f64, t3565: f64, t3566: f64, t3567: f64, t3690: f64, t3694: f64, t3702: f64, t3704: f64, t3707: f64, t3722: f64, t3725: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t3781 = piecewise5(t323, t3741 + t3743, t331, t3771, -0.21e1_f64 * t3549 * t3675 - 0.105e1_f64 * t855 * t3774 * t352 - 0.1575e1_f64 * t3556 * t3675);
    let t3787 = -t3564 + t3565 - t3566 - t3567 - 0.72042316457491791901e-3_f64 * t3690 + 0.30487649791575028312e-3_f64 * t3694 - t3702 - t3704 + t3707 - t3722 + t3725;
    (t3781, t3787)
}
