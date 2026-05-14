//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 483/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk483<F: Float>(t53: F, t5455: F, t5456: F, t5458: F, t5472: F, t1439: F, t453: F, t1156: F, t592: F, t1144: F, t589: F, t4396: F, t521: F, t50: F, t983: F, t1375: F, t1378: F, t154: F, t437: F, t5328: F, t814: F, t913: F, t916: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t5474 = t5455 + t5456 + t5458 + t5472;
    let t5477 = t1439 * t453;
    let t5480 = t592 * t1156;
    let t5491 = t589 * t1144;
    let t5498 = t4396 * t521;
    let t5501 = t983 * t50;
    let t5511 = piecewise3(t54, 0.0, 8.0 / 27.0 * t5498 * t913 - 8.0 / 9.0 * t5501 * t5328 - 2.0 / 9.0 * t1375 * t916 + 4.0 / 3.0 * t437 * t814 - 4.0 * t1378 * t154);
    (t5474, t5477, t5480, t5491, t5511)
}
