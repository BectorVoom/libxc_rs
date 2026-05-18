//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 533/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk533<F: Float>(t1705: F, t2404: F, t2408: F, t4864: F, t4881: F, t1248: F, t2364: F, t4889: F, t2422: F, t45: F, t2430: F, t4928: F) -> (F, F, F, F, F, F) {
    let t7091 = t2404 * t1705;
    let t7099 = t4864 * t2408;
    let t7115 = t4881 * t2408;
    let t7122 = t1248 * t4889 * t2364;
    let t7151 = t45 * t2422;
    let t7156 = t4928 * t2430;
    (t7091, t7099, t7115, t7122, t7151, t7156)
}
