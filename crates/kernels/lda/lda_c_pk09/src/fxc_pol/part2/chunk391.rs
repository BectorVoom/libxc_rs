//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 391/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk391<F: Float>(t1918: F, t1919: F, t454: F, t1805: F, t1830: F, t1834: F, t1840: F, t1842: F, t1844: F, t1847: F, t1849: F, t1855: F, t1856: F, t1859: F, t1873: F, t1879: F, t1898: F, t1901: F, t1904: F, t1907: F, t455: F) -> (F, F, F) {
    let t1920 = t1918 * t1919;
    let t1921 = t454 * t1920;
    let t1924 = -t1830 + t1834 + t1840 - F::new(7.108175748183851) * t1842 * t1844 + F::new(7.108175748183851) * t1847 * t1849 + t1855 + F::new(2.427516195194328) * t1856 * t1805 - F::new(1.8805371096875316) * t1859 * t1805 + F::new(1.8805371096875316) * t1873 * t455 - t1879 - F::new(2.2140749178833072) * t1898 * t455 - F::new(18.635258017632964) * t1901 * t1805 + F::new(0.04115066352984959) * t1904 * t1907 - F::new(0.04115066352984959) * t1904 * t1921;
    (t1920, t1921, t1924)
}
