//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1018/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1018(t11045: f64, t6442: f64, t11023: f64, t9700: f64, t2723: f64, t9704: f64, t11004: f64, t11039: f64, t11042: f64, t1193: f64, t1197: f64, t1713: f64, t2711: f64, t2724: f64, t620: f64, t6409: f64) -> f64 {
    let t11046 = t11045 * t6442;
    let t11049 = t11023 * t9700;
    let t11052 = t2723 * t9704;
    let t11055 = t11004 * t1193 + t2711 * t620 * t1197 + t11039 - t11042 + 1.28_f64 * t6409 * t2724 - 1.28_f64 * t1713 * t11046 + 2.56_f64 * t1713 * t11049 - 1.28_f64 * t1713 * t11052;
    t11055
}
