//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1350/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1350(t225: f64, t24873: f64, t1235: f64, t7319: f64, t24705: f64, t491: f64, t24574: f64, t24639: f64, t24568: f64, t24634: f64, t1090: f64, t11918: f64, t1238: f64, t1252: f64, t2154: f64, t2155: f64, t24589: f64, t24601: f64, t24868: f64, t24880: f64, t3487: f64, t3598: f64, t3600: f64, t45375: f64, t7283: f64, t7287: f64, t7300: f64, t7301: f64, t85687: f64) -> f64 {
    let t85717 = t24873 * t225;
    let t85724 = t7319 * t1235;
    let t85728 = t24705 * t491;
    let t85733 = t24574 * t24639;
    let t85739 = t24574 * t24568;
    let t85741 = t24574 * t24634;
    let t85749 = 6.0_f64 * t24880 * t3600 - 6.0_f64 * t85717 * t1252 - 0.16449340668482264365e-1_f64 * t24589 * t24601 * t85687 * t1090 + 0.16449340668482264365e-1_f64 * t24589 * t85724 * t7287 + 0.82246703342411321826e-2_f64 * t24589 * t85728 * t7287 - t45375 * t2155 + 0.16449340668482264365e-1_f64 * t85733 - 0.82246703342411321825e-2_f64 * t7283 * t7300 * t7301 * t11918 - 0.16449340668482264365e-1_f64 * t85739 - 0.54831135561607547883e-2_f64 * t85741 + 2.0_f64 * t1238 * t3598 * t2154 * t11918 - 3.0_f64 * t3487 * t24868;
    t85749
}
