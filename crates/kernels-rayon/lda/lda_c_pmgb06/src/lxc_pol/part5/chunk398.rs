//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 398/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk398(t1902: f64, t439: f64, t1676: f64, t1679: f64, t1682: f64, t1689: f64, t1692: f64, t1700: f64, t1703: f64, t1816: f64, t1839: f64, t1850: f64, t1852: f64, t1854: f64, t1876: f64, t1878: f64, t1889: f64, t1896: f64, t1900: f64) -> (f64, f64) {
    let t1904 = t439 * t1902 / 27.0_f64;
    let t1905 = t1816 + t1839 + t1850 + t1852 + t1854 + t1876 + t1878 + t1889 + 2.0_f64 / 9.0_f64 * t1676 + t1679 - t1682 + t1689 / 3.0_f64 + 0.06077777777777778_f64 * t1692 + t1700 + t1703 - t1896 - t1900 + t1904;
    (t1904, t1905)
}
