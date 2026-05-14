//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1170/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1170<F: Float>(t31493: F, t31510: F, t10009: F, t10013: F, t11166: F, t11214: F, t18887: F, t18889: F, t22564: F, t2291: F, t27699: F, t27771: F, t27839: F, t27912: F, t3088: F, t3107: F, t3121: F, t3140: F, t31456: F, t31458: F, t31461: F, t31464: F, t31472: F, t3807: F, t8120: F, t8211: F, t870: F, t882: F, t890: F) -> (F, F) {
    let t31511 = t31493 + t31510;
    let t31517 = 0.19964560303604640732e6 * t18887 * t11166 * t18889 * t870 - 6.0 * t27771 * t3088 + 0.96491876992155210402e2 * t27839 * t3107 - 6.0 * t8211 * t10009 + 0.96491876992155210402e2 * t8120 * t10013 - 0.35089341735807877242e1 * t27912 * t3121 + 0.51947577317044391276e2 * t27699 * t3140 - t31456 + t31458 + t31461 - t31464 - 0.19751673498613801407e-1 * t31472 + 0.5848223622634646207e0 * t2291 * t11214 + 0.5848223622634646207e0 * t882 * t31511 * t890 - 0.35089341735807877242e1 * t22564 * t3807;
    (t31511, t31517)
}
