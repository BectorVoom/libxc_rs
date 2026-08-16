//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 504/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk504<F: Float>(t5455: F, t5456: F, t5458: F, t5472: F, t1439: F, t453: F, t1156: F, t592: F, t1144: F, t589: F, t4396: F, t521: F) -> (F, F, F, F, F) {
    let t5474 = t5455 + t5456 + t5458 + t5472;
    let t5477 = t1439 * t453;
    let t5480 = t592 * t1156;
    let t5491 = t589 * t1144;
    let t5498 = t4396 * t521;
    (t5474, t5477, t5480, t5491, t5498)
}
