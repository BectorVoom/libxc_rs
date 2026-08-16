//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1048/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1048(t3438: f64, t37355: f64, t10978: f64, t10979: f64, t2317: f64, t597: f64, t10673: f64, t10682: f64, t2279: f64, t357: f64, t10647: f64, t10652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37356 = t3438 * t37355;
    let t37358 = t10978 * t10979 * t2317 * t37356;
    let t37359 = 0.13010691197123848594e-3_f64 * t37358;
    let t37360 = t597 * t37355;
    let t37362 = t10673 * t10682 * t37360;
    let t37364 = t2279 * t357;
    let t37365 = t37364 * t10647;
    let t37366 = t37365 * t10652;
    (t37359, t37360, t37362, t37364, t37365, t37366)
}
