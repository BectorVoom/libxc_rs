//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1283/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1283<F: Float>(t3546: F, t11159: F, t978: F, t2559: F, t4278: F, t2537: F, t11079: F, t11089: F, t11139: F, t1422: F, t21729: F, t2538: F, t2539: F, t2540: F, t2555: F, t2560: F, t2562: F, t2563: F, t25633: F, t25648: F, t2578: F, t2599: F, t2601: F, t30041: F, t3527: F, t3547: F, t4284: F, t4297: F, t4311: F, t4324: F, t7002: F, t7109: F, t9042: F, t9045: F, t9079: F, t9205: F, t9210: F, t9242: F, t9248: F, t987: F, t988: F) -> (F,) {
    let t30047 = t3546 * t3546;
    let t30056 = t11159 * t978;
    let t30061 = t4278 * t2559;
    let t30071 = t4278 * t2537;
    let t30098 = 0.64327917994770140268e2 * t2560 * t30047 * t2562 + 2.0 * t3527 * t9242 - 4.0 * t2538 * t30047 * t987 + 2.0 * t30056 * t988 + 1.0 * t11079 * t2555 + 0.32163958997385070134e2 * t30061 * t2563 + 2.0 * t25633 * t1422 + 4.0 * t9205 * t3547 + 0.34631718211362927518e2 * t2599 * t30041 * t2601 - 2.0 * t30071 * t2540 + 0.35089341735807877242e1 * t2599 * t4324 * t2578 - 0.14035736694323150897e2 * t7109 * t4311 * t2578 + 6.0 * t2560 * t4297 * t2539 - 24.0 * t7002 * t4284 * t2539 - 0.10389515463408878255e3 * t7109 * t11139 * t2578 - 0.12304822629859687989e5 * t21729 * t11089 * t2578 - 0.38596750796862084161e3 * t25648 * t9079 + 0.70178683471615754484e1 * t9248 * t9042 + 12.0 * t9210 * t9045;
    (t30098,)
}
