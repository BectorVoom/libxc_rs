//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 468/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk468<F: Float>(t1368: F, t1370: F, t1374: F, t1379: F, t1816: F, t1839: F, t1850: F, t1852: F, t1854: F, t1876: F, t1878: F, t1889: F, t1896: F, t1900: F, t1904: F, t1910: F, t1914: F) -> F {
    let t2341 = t1368 + F::new(0.10821041362364843) * t1370 + t1374 + t1379 + t1816 + t1839 + t1850 + t1852 + t1854 + t1876 + t1878 + t1889 - t1896 - t1900 + t1904 - t1910 - t1914;
    t2341
}
