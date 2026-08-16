//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 448/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk448(t107: f64, t110: f64, t122: f64, t1658: f64, t1659: f64, t1661: f64, t1672: f64, t1674: f64, t1741: f64, t1743: f64, t1796: f64, t1799: f64, t1804: f64, t1808: f64, t1813: f64, t199: f64, t202: f64, t2116: f64, t2122: f64, t2164: f64, t399: f64, t566: f64, t795: f64, t84: f64, t868: f64) -> f64 {
    let t2168 = -t1658 + 0.0837628205355044_f64 * t1659 + 0.0837628205355044_f64 * t1661 + 0.0837628205355044_f64 * t1796 - 0.0837628205355044_f64 * t1799 * t199 - 0.0837628205355044_f64 * t795 * t566 + 0.0837628205355044_f64 * t1804 - 0.0837628205355044_f64 * t399 * t868 - 0.0837628205355044_f64 * t84 * t1808 - t1672 + 0.019897291109174608_f64 * t1674 + 0.019897291109174608_f64 * t1813 - 0.011938374665504766_f64 * t122 * t202 * t2116 + t1741 - 0.5694518669548363_f64 * t1743 - 0.5694518669548363_f64 * t2122 + 0.42708890021612717_f64 * t107 * t110 * t2164;
    t2168
}
