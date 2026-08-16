//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 898/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk898(t6452: f64, t738: f64, t6455: f64, t743: f64, t6458: f64, t733: f64, t10033: f64, t10093: f64, t10099: f64, t10108: f64, t13472: f64, t13473: f64, t13492: f64, t13493: f64, t13499: f64, t13502: f64, t13532: f64, t13535: f64, t13567: f64, t18508: f64, t19396: f64, t19399: f64, t3061: f64, t3150: f64) -> f64 {
    let t19416 = t738 * t6452;
    let t19418 = t743 * t6455;
    let t19420 = t733 * t6458;
    let t19422 = -0.62154466893555682512e-3_f64 * t10099 * t19396 + 0.62154466893555682512e-3_f64 * t13567 * t19399 - 0.23911438650126355246e-1_f64 * t3061 * t18508 + 0.15538616723388920628e-3_f64 * t3150 * t18508 + 0.71734315950379065738e-1_f64 * t10093 * t19396 - 0.95645754600505420984e-1_f64 * t10108 * t19399 + 0.39210208333333333333e-4_f64 * t10033 + t13472 + 0.31368166666666666667e-4_f64 * t13473 - t13492 - 0.31226666666666666667e-2_f64 * t13493 - 0.47822877300252710492e-1_f64 * t13499 + 0.41436311262370455008e-3_f64 * t13502 + 0.52833333333333333332e-2_f64 * t13532 + t13535 + 0.26416666666666666667e-2_f64 * t19416 + 0.23526125e-4_f64 * t19418 - 0.9368e-2_f64 * t19420;
    t19422
}
