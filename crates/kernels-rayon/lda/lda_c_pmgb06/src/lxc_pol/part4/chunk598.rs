//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 598/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk598(t2504: f64, t2532: f64, t2568: f64, t2657: f64, t107: f64, t110: f64, t122: f64, t1658: f64, t1672: f64, t1741: f64, t1796: f64, t1804: f64, t1813: f64, t199: f64, t202: f64, t2122: f64, t2407: f64, t2422: f64, t2454: f64, t795: f64, t84: f64, t868: f64) -> (f64, f64) {
    let t2659 = t2504 + t2532 + t2568 + t2657;
    let t2667 = -t1658 + 0.1675256410710088_f64 * t1796 + 0.1675256410710088_f64 * t1804 - 0.0837628205355044_f64 * t2454 * t199 - 0.1675256410710088_f64 * t795 * t868 - 0.0837628205355044_f64 * t84 * t2422 - t1672 + 0.039794582218349216_f64 * t1813 - 0.011938374665504766_f64 * t122 * t202 * t2659 + t1741 - 1.1389037339096726_f64 * t2122 + 0.42708890021612717_f64 * t107 * t110 * t2407;
    (t2659, t2667)
}
