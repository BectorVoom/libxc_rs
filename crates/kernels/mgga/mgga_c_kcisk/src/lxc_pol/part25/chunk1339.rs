//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1339/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1339<F: Float>(t17972: F, t33121: F, t33106: F, t34316: F, t17802: F, t1800: F, t17861: F, t654: F, t9705: F, t117347: F, t117350: F, t117352: F, t117354: F, t117356: F, t117358: F, t117360: F, t117363: F, t117365: F, t117367: F, t117370: F, t117373: F, t117375: F, t117377: F) -> (F, F, F, F, F) {
    let t117379 = t33121 * t17972;
    let t117381 = t34316 * t33106;
    let t117383 = t1800 * t17802;
    let t117385 = t17861 * t654;
    let t117386 = t117385 * t9705;
    let t117388 = -t117347 / 6.0 - 2.0 / 9.0 * t117350 + t117352 / 24.0 - t117354 / 9.0 - t117356 / 16.0 - 3.0 / 8.0 * t117358 + t117360 / 6.0 + 3.0 / 64.0 * t117363 - t117365 / 72.0 + t117367 / 48.0 + t117370 / 6.0 - t117373 / 16.0 + t117375 / 27.0 + t117377 / 18.0 + t117379 / 96.0 - t117381 / 72.0 + t117383 / 54.0 - t117386 / 8.0;
    (t117379, t117381, t117383, t117386, t117388)
}
