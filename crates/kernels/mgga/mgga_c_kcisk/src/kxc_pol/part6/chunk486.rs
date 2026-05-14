//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 486/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk486<F: Float>(t397: F, t4889: F, t786: F, t782: F, t4597: F, t2020: F, t695: F, t1849: F, t2019: F, t785: F, t657: F, t2040: F, t801: F, t798: F, t2059: F, t3277: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5477 = t397 * t4889 * t786;
    let t5479 = 0.59969295720591057378e-2 * t782 * t5477;
    let t5486 = t786 * t4597;
    let t5491 = t2020 * t695;
    let t5497 = t786 * t1849;
    let t5507 = 1.0 / t2019 / t785;
    let t5508 = t657 * t5507;
    let t5531 = 1.0 / t2040 / t801;
    let t5532 = t798 * t5531;
    let t5562 = t3277 * t2059;
    (t5477, t5479, t5486, t5491, t5497, t5507, t5508, t5531, t5532, t5562)
}
