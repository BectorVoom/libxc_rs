//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1344/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1344<F: Float>(t117306: F, t117308: F, t117311: F, t117313: F, t117315: F, t117317: F, t117319: F, t117321: F, t117323: F, t117325: F, t117328: F, t117330: F, t117332: F, t117335: F, t117337: F, t117339: F, t117341: F, t117343: F) -> (F,) {
    let t117479 = 0.375e0 * t117306 + 0.9375e-1 * t117308 + 0.1875e0 * t117311 + 0.20234375e-1 * t117313 + 0.20234375e-1 * t117315 - 0.125e0 * t117317 - 0.9375e-1 * t117319 + 0.625e-1 * t117321 - 0.125e0 * t117323 - 0.21583333333333333334e0 * t117325 + 0.125e0 * t117328 - 0.41666666666666666666e-1 * t117330 - 0.4046875e-1 * t117332 - 0.17986111111111111111e-1 * t117335 + 0.4046875e-1 * t117337 - 0.26979166666666666667e-1 * t117339 - 0.10791666666666666667e0 * t117341 - 0.625e-1 * t117343;
    (t117479,)
}
