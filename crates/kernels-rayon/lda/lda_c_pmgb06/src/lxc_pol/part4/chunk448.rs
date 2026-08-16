//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 448/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk448(t1704: f64, t1705: f64, t1707: f64, t1733: f64, t107: f64, t1180: f64, t290: f64, t410: f64, t701: f64, t110: f64, t1126: f64, t1200: f64, t122: f64, t1338: f64, t1658: f64, t1659: f64, t1661: f64, t1672: f64, t1674: f64, t199: f64, t202: f64, t399: f64, t566: f64, t84: f64) -> (f64, f64, f64, f64) {
    let t1735 = t1704 + t1705 + t1707 + t1733;
    let t1741 = 1.328721022894618_f64 * t107 * t1180 * t290;
    let t1743 = t107 * t410 * t701;
    let t1748 = -t1658 + 0.1675256410710088_f64 * t1659 + 0.1675256410710088_f64 * t1661 - 0.0837628205355044_f64 * t1338 * t199 - 0.1675256410710088_f64 * t399 * t566 - 0.0837628205355044_f64 * t84 * t1200 - t1672 + 0.039794582218349216_f64 * t1674 - 0.011938374665504766_f64 * t122 * t202 * t1735 + t1741 - 1.1389037339096726_f64 * t1743 + 0.42708890021612717_f64 * t107 * t110 * t1126;
    (t1735, t1741, t1743, t1748)
}
