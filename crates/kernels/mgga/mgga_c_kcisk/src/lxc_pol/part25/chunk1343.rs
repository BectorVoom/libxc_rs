//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1343/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1343<F: Float>(t117305: F, t117345: F, t117388: F, t117431: F, t1908: F, t117268: F, t117270: F, t117272: F, t117274: F, t117276: F, t117278: F, t117280: F, t117282: F, t117284: F, t117286: F, t117288: F, t117290: F, t117292: F, t117295: F, t117297: F, t117299: F, t117301: F, t117303: F) -> (F, F) {
    let t117434 = t1908 * (t117305 + t117345 + t117388 + t117431);
    let t117460 = 0.27777777777777777777e-1 * t117268 + 0.33333333333333333334e0 * t117270 - 0.10791666666666666667e0 * t117272 + 0.21583333333333333334e0 * t117274 + 0.375e0 * t117276 - 0.125e0 * t117278 + 0.17986111111111111111e-1 * t117280 - 0.1875e0 * t117282 + 0.20234375e-1 * t117284 - 0.53958333333333333334e-1 * t117286 + 0.4046875e-1 * t117288 - 0.89930555555555555557e-2 * t117290 - 0.26979166666666666667e-1 * t117292 - 0.5e0 * t117295 + 0.125e0 * t117297 - 0.625e-1 * t117299 - 0.4046875e-1 * t117301 + 0.28777777777777777778e0 * t117303;
    (t117434, t117460)
}
