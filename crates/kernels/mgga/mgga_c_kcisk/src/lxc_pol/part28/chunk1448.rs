//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1448/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1448<F: Float>(t122289: F, t122291: F, t122293: F, t122295: F, t122297: F, t122299: F, t122301: F, t122303: F, t122305: F, t122307: F, t122309: F, t122337: F, t122339: F, t122341: F, t122344: F, t122347: F, t122349: F, t122351: F, t122353: F, t122355: F, t122357: F, t122359: F) -> (F, F) {
    let t123414 = 0.61111111111111111111e0 * t122289 + 0.25e0 * t122291 - 0.53958333333333333333e-1 * t122293 + 0.33333333333333333333e0 * t122295 - 0.28777777777777777779e0 * t122297 + 0.59953703703703703705e-2 * t122299 - 0.28777777777777777778e0 * t122301 + 0.41666666666666666667e-1 * t122303 + 0.89930555555555555557e-2 * t122305 + 0.4046875e-1 * t122307 + 0.26979166666666666667e-1 * t122309;
    let t123438 = -0.91666666666666666667e0 * t122337 + 0.28777777777777777778e0 * t122339 - 0.20833333333333333333e-1 * t122341 + 0.625e-1 * t122344 - 0.17986111111111111111e-1 * t122347 - 0.1875e0 * t122349 - 0.89930555555555555557e-2 * t122351 - 0.809375e-1 * t122353 - 0.5625e0 * t122355 + 0.17986111111111111111e-1 * t122357 - 0.20833333333333333333e-1 * t122359;
    (t123414, t123438)
}
