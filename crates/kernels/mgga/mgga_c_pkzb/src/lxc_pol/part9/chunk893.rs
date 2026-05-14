//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 893/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk893<F: Float>(t1096: F, t1917: F, t1108: F, t1956: F, t1979: F, t2848: F, t721: F, t1971: F, t2852: F, t1938: F, t1977: F, t2801: F, t2820: F, t5825: F, t5897: F, t7268: F, t7271: F, t7274: F, t7277: F, t7281: F, t7284: F, t7288: F) -> (F, F, F, F, F, F) {
    let t7293 = t1096 * t1917;
    let t7296 = t1108 * t1956;
    let t7299 = t2848 * t1979;
    let t7300 = t7299 * t721;
    let t7303 = t2852 * t1971;
    let t7306 = -t7268 + t7271 + t7274 + t7277 - t7281 - t7284 - t7288 - 4.0 * t5897 * t2801 + 0.64327917994770140268e2 * t5825 * t2820 + 6.0 * t1938 * t7293 + 0.35089341735807877242e1 * t1977 * t7296 + 0.34631718211362927518e2 * t1977 * t7300 + 0.17315859105681463759e2 * t1977 * t7303;
    (t7293, t7296, t7299, t7300, t7303, t7306)
}
