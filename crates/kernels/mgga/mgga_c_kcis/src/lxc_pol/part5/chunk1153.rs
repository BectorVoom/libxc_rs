//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1153/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1153<F: Float>(t10033: F, t10093: F, t10099: F, t10108: F, t13472: F, t13473: F, t13492: F, t13493: F, t13499: F, t13502: F, t13532: F, t13535: F, t13567: F, t18508: F, t19396: F, t19399: F, t19416: F, t19418: F, t19420: F, t3061: F, t3150: F) -> F {
    let t19422 = -F::cast_from(0.62154466893555682512e-3_f64) * t10099 * t19396 + F::cast_from(0.62154466893555682512e-3_f64) * t13567 * t19399 - F::cast_from(0.23911438650126355246e-1_f64) * t3061 * t18508 + F::cast_from(0.15538616723388920628e-3_f64) * t3150 * t18508 + F::cast_from(0.71734315950379065738e-1_f64) * t10093 * t19396 - F::cast_from(0.95645754600505420984e-1_f64) * t10108 * t19399 + F::cast_from(0.39210208333333333333e-4_f64) * t10033 + t13472 + F::cast_from(0.31368166666666666667e-4_f64) * t13473 - t13492 - F::cast_from(0.31226666666666666667e-2_f64) * t13493 - F::cast_from(0.47822877300252710492e-1_f64) * t13499 + F::cast_from(0.41436311262370455008e-3_f64) * t13502 + F::cast_from(0.52833333333333333332e-2_f64) * t13532 + t13535 + F::cast_from(0.26416666666666666667e-2_f64) * t19416 + F::new(0.23526125e-4) * t19418 - F::new(0.9368e-2) * t19420;
    t19422
}
