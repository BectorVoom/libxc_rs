//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1295/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1295(t31493: f64, t31510: f64, t10009: f64, t10013: f64, t11166: f64, t11214: f64, t18887: f64, t18889: f64, t22564: f64, t2291: f64, t27699: f64, t27771: f64, t27839: f64, t27912: f64, t3088: f64, t3107: f64, t3121: f64, t3140: f64, t31456: f64, t31458: f64, t31461: f64, t31464: f64, t31472: f64, t3807: f64, t8120: f64, t8211: f64, t870: f64, t882: f64, t890: f64) -> (f64, f64) {
    let t31511 = t31493 + t31510;
    let t31517 = 0.19964560303604640732e6_f64 * t18887 * t11166 * t18889 * t870 - 6.0_f64 * t27771 * t3088 + 0.96491876992155210402e2_f64 * t27839 * t3107 - 6.0_f64 * t8211 * t10009 + 0.96491876992155210402e2_f64 * t8120 * t10013 - 0.35089341735807877242e1_f64 * t27912 * t3121 + 0.51947577317044391276e2_f64 * t27699 * t3140 - t31456 + t31458 + t31461 - t31464 - 0.19751673498613801407e-1_f64 * t31472 + 0.5848223622634646207e0_f64 * t2291 * t11214 + 0.5848223622634646207e0_f64 * t882 * t31511 * t890 - 0.35089341735807877242e1_f64 * t22564 * t3807;
    (t31511, t31517)
}
