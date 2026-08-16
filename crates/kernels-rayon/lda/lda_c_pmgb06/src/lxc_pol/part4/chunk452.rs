//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 452/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk452(t1773: f64, t302: f64, t1316: f64, t1317: f64, t1323: f64, t1324: f64, t1655: f64, t1748: f64, t1750: f64, t1753: f64, t1755: f64, t1760: f64, t1765: f64, t1772: f64, t295: f64, t297: f64, t346: f64, t388: f64, t61: f64) -> (f64, f64) {
    let t1775 = 0.05321881782335382_f64 * t1773 * t302;
    let t1776 = 6.0_f64 * t1316 * t388 * t1317 - t346 * t1323 * t1324 + t1655 * t61 + t1748 * t295 + 0.039914113367515366_f64 * t1750 + t1753 - 0.01197423401025461_f64 * t297 * t1755 - 0.02394846802050922_f64 * t1760 - t1765 - t1772 - t1775;
    (t1775, t1776)
}
