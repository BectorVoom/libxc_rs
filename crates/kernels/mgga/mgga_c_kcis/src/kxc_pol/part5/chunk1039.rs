//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1039/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1039<F: Float>(t331: F, t6310: F, t6272: F, t829: F, t1646: F, t167: F, t6452: F, t738: F, t6455: F, t743: F, t6458: F, t733: F, t10033: F, t10093: F, t10099: F, t10108: F, t13472: F, t13473: F, t13492: F, t13493: F, t13499: F, t13502: F, t13532: F, t13535: F, t13567: F, t18508: F, t3061: F, t3150: F) -> (F, F, F, F) {
    let t19381 = t331 * t6310;
    let t19396 = t6272 * t829;
    let t19399 = t1646 * t167;
    let t19416 = t738 * t6452;
    let t19418 = t743 * t6455;
    let t19420 = t733 * t6458;
    let t19422 = -0.62154466893555682512e-3 * t10099 * t19396 + 0.62154466893555682512e-3 * t13567 * t19399 - 0.23911438650126355246e-1 * t3061 * t18508 + 0.15538616723388920628e-3 * t3150 * t18508 + 0.71734315950379065738e-1 * t10093 * t19396 - 0.95645754600505420984e-1 * t10108 * t19399 + 0.39210208333333333333e-4 * t10033 + t13472 + 0.31368166666666666667e-4 * t13473 - t13492 - 0.31226666666666666667e-2 * t13493 - 0.47822877300252710492e-1 * t13499 + 0.41436311262370455008e-3 * t13502 + 0.52833333333333333332e-2 * t13532 + t13535 + 0.26416666666666666667e-2 * t19416 + 0.23526125e-4 * t19418 - 0.9368e-2 * t19420;
    (t19381, t19396, t19399, t19422)
}
