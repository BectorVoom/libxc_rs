//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1345/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1345<F: Float>(t117347: F, t117350: F, t117352: F, t117354: F, t117356: F, t117358: F, t117360: F, t117363: F, t117365: F, t117367: F, t117370: F, t117373: F, t117375: F, t117377: F, t117379: F, t117381: F, t117383: F, t117386: F) -> (F,) {
    let t117499 = -0.25e0 * t117347 - 0.33333333333333333334e0 * t117350 + 0.625e-1 * t117352 - 0.16666666666666666667e0 * t117354 - 0.9375e-1 * t117356 - 0.5625e0 * t117358 + 0.25e0 * t117360 + 0.12140625e0 * t117363 - 0.20833333333333333333e-1 * t117365 + 0.53958333333333333334e-1 * t117367 + 0.25e0 * t117370 - 0.9375e-1 * t117373 + 0.55555555555555555557e-1 * t117375 + 0.14388888888888888889e0 * t117377 + 0.26979166666666666667e-1 * t117379 - 0.20833333333333333333e-1 * t117381 + 0.47962962962962962963e-1 * t117383 - 0.1875e0 * t117386;
    (t117499,)
}
