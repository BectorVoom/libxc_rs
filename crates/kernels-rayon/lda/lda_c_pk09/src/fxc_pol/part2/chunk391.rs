//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 391/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk391(t1918: f64, t1919: f64, t454: f64, t1805: f64, t1830: f64, t1834: f64, t1840: f64, t1842: f64, t1844: f64, t1847: f64, t1849: f64, t1855: f64, t1856: f64, t1859: f64, t1873: f64, t1879: f64, t1898: f64, t1901: f64, t1904: f64, t1907: f64, t455: f64) -> (f64, f64, f64) {
    let t1920 = t1918 * t1919;
    let t1921 = t454 * t1920;
    let t1924 = -t1830 + t1834 + t1840 - 7.108175748183851_f64 * t1842 * t1844 + 7.108175748183851_f64 * t1847 * t1849 + t1855 + 2.427516195194328_f64 * t1856 * t1805 - 1.8805371096875316_f64 * t1859 * t1805 + 1.8805371096875316_f64 * t1873 * t455 - t1879 - 2.2140749178833072_f64 * t1898 * t455 - 18.635258017632964_f64 * t1901 * t1805 + 0.04115066352984959_f64 * t1904 * t1907 - 0.04115066352984959_f64 * t1904 * t1921;
    (t1920, t1921, t1924)
}
