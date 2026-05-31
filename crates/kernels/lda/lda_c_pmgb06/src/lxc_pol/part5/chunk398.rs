//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 398/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk398<F: Float>(t1902: F, t439: F, t1676: F, t1679: F, t1682: F, t1689: F, t1692: F, t1700: F, t1703: F, t1816: F, t1839: F, t1850: F, t1852: F, t1854: F, t1876: F, t1878: F, t1889: F, t1896: F, t1900: F) -> (F, F) {
    let t1904 = t439 * t1902 / F::cast_from(27.0_f64);
    let t1905 = t1816 + t1839 + t1850 + t1852 + t1854 + t1876 + t1878 + t1889 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1676 + t1679 - t1682 + t1689 / F::cast_from(3.0_f64) + F::cast_from(0.06077777777777778_f64) * t1692 + t1700 + t1703 - t1896 - t1900 + t1904;
    (t1904, t1905)
}
