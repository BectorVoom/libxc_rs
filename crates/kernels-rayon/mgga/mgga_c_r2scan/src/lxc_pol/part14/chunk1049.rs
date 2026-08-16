//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1049/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1049(t37355: f64, t597: f64, t10673: f64, t10682: f64, t2279: f64, t357: f64, t10647: f64, t10652: f64, t2289: f64, t2281: f64, t10935: f64, t2065: f64, t3446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37360 = t597 * t37355;
    let t37362 = t10673 * t10682 * t37360;
    let t37364 = t2279 * t357;
    let t37365 = t37364 * t10647;
    let t37366 = t37365 * t10652;
    let t37368 = t2289 * t357;
    let t37369 = t37368 * t10647;
    let t37370 = t37369 * t10652;
    let t37372 = t2281 * t357;
    let t37373 = t37372 * t10647;
    let t37374 = t37373 * t10652;
    let t37377 = t3446 * t10935 * t2065;
    (t37360, t37362, t37364, t37365, t37366, t37368, t37369, t37370, t37372, t37373, t37374, t37377)
}
