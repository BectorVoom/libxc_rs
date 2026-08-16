//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 797/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk797<F: Float>(t13194: F, t1841: F, t13200: F, t13182: F, t29439: F, t33289: F, t7810: F, t9889: F, t13055: F, t28073: F, t32840: F, t3295: F, t9805: F) -> (F, F, F, F, F, F) {
    let t43095 = t1841 * t13194;
    let t43098 = t1841 * t13200;
    let t43100 = t29439 * t13182;
    let t43363 = t7810 * t33289 * t9889;
    let t43370 = t28073 * t13055;
    let t43373 = t9805 * t32840 * t3295;
    (t43095, t43098, t43100, t43363, t43370, t43373)
}
