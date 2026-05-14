//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 927/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk927<F: Float>(t10932: F, t179: F, t5634: F, t2889: F, t3515: F, t2888: F, t1123: F, t3638: F, t301: F, t5934: F, t758: F, t10833: F, t1976: F, t722: F, t730: F, t2852: F, t9351: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10934 = t179 * t5634 * t10932;
    let t10937 = t2889 * t3515;
    let t10938 = t2888 * t10937;
    let t10942 = t3638 * t1123;
    let t10943 = t301 * t10942;
    let t10944 = t10943 * t5934;
    let t10945 = t758 * t10944;
    let t10949 = t1976 * t10833 * t722;
    let t10951 = 0.35089341735807877242e1 * t730 * t10949;
    let t10952 = t9351 * t2852;
    (t10934, t10937, t10938, t10942, t10943, t10944, t10945, t10949, t10951, t10952)
}
