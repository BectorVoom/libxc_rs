//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 604/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk604(t227: f64, t297: f64, t8459: f64, t294: f64, t2452: f64, t807: f64, t2356: f64, t2361: f64, t2671: f64, t7718: f64, t565: f64, t806: f64, t564: f64, sigma2: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t8460 = t297 * t8459;
    let t8461 = t294 * t8460;
    let t8463 = 1.0_f64 / t2452;
    let t8464 = sigma2 * t8463;
    let t8465 = t8464 * t807;
    let t8467 = t2356 * t2361;
    let t8469 = t2356 * t2671;
    let t8471 = piecewise3(t228, 0.0_f64, t7718);
    let t8472 = t565 * t8471;
    let t8473 = t8472 * t806;
    let t8474 = t564 * t8473;
    (t8461, t8464, t8465, t8467, t8469, t8472, t8473, t8474)
}
