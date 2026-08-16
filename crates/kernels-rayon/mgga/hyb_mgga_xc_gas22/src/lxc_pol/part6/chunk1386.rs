//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1386/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1386(t3546: f64, t11159: f64, t978: f64, t2559: f64, t4278: f64, t2537: f64, t11079: f64, t11089: f64, t11139: f64, t1422: f64, t21729: f64, t2538: f64, t2539: f64, t2540: f64, t2555: f64, t2560: f64, t2562: f64, t2563: f64, t25633: f64, t25648: f64, t2578: f64, t2599: f64, t2601: f64, t30041: f64, t3527: f64, t3547: f64, t4284: f64, t4297: f64, t4311: f64, t4324: f64, t7002: f64, t7109: f64, t9042: f64, t9045: f64, t9079: f64, t9205: f64, t9210: f64, t9242: f64, t9248: f64, t987: f64, t988: f64) -> f64 {
    let t30047 = t3546 * t3546;
    let t30056 = t11159 * t978;
    let t30061 = t4278 * t2559;
    let t30071 = t4278 * t2537;
    let t30098 = 0.64327917994770140268e2_f64 * t2560 * t30047 * t2562 + 2.0_f64 * t3527 * t9242 - 4.0_f64 * t2538 * t30047 * t987 + 2.0_f64 * t30056 * t988 + 1.0_f64 * t11079 * t2555 + 0.32163958997385070134e2_f64 * t30061 * t2563 + 2.0_f64 * t25633 * t1422 + 4.0_f64 * t9205 * t3547 + 0.34631718211362927518e2_f64 * t2599 * t30041 * t2601 - 2.0_f64 * t30071 * t2540 + 0.35089341735807877242e1_f64 * t2599 * t4324 * t2578 - 0.14035736694323150897e2_f64 * t7109 * t4311 * t2578 + 6.0_f64 * t2560 * t4297 * t2539 - 24.0_f64 * t7002 * t4284 * t2539 - 0.10389515463408878255e3_f64 * t7109 * t11139 * t2578 - 0.12304822629859687989e5_f64 * t21729 * t11089 * t2578 - 0.38596750796862084161e3_f64 * t25648 * t9079 + 0.70178683471615754484e1_f64 * t9248 * t9042 + 12.0_f64 * t9210 * t9045;
    t30098
}
