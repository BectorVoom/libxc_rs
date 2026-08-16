//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2559/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2559(t21886: f64, t3359: f64, t11350: f64, t1136: f64, t11420: f64, t15126: f64, t15136: f64, t15146: f64, t18609: f64, t18612: f64, t18616: f64, t18619: f64, t18647: f64, t18650: f64, t18651: f64, t21854: f64, t21855: f64, t21887: f64, t21890: f64, t3332: f64, t3357: f64, t44177: f64, t44179: f64, t44361: f64, t4819: f64, t4840: f64, t4862: f64, t51604: f64, t51680: f64, t6056: f64, t63454: f64, t63602: f64) -> f64 {
    let t71729 = t21886 * t3359;
    let t71752 = 0.1929837539843104208e3_f64 * t15146 * t18647 + 0.62071215503128080361e4_f64 * t51604 * t18651 + 0.11579025239058625248e4_f64 * t11350 * t21855 * t1136 - 0.57895126195293126243e3_f64 * t11420 * t6056 * t4819 - 0.24828486201251232145e5_f64 * t44361 * t21890 * t1136 - 2.0_f64 * t3332 * t21887 * t1136 + 0.32163958997385070134e2_f64 * t3357 * t71729 * t1136 + 0.6207121550312808036e4_f64 * t11350 * t18650 * t4819 + 0.19964560303604640732e6_f64 * t44177 * t21854 * t44179 * t1136 - 0.35089341735807877242e1_f64 * t63454 * t4840 + 0.51947577317044391276e2_f64 * t63602 * t4862 - 0.35089341735807877242e1_f64 * t15136 * t18612 + 0.51947577317044391276e2_f64 * t15126 * t18616 - 0.31168546390226634765e3_f64 * t51680 * t18609 + 0.10389515463408878255e3_f64 * t15126 * t18619;
    t71752
}
