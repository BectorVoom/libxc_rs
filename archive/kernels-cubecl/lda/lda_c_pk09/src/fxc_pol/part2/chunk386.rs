//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 386/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk386<F: Float>(t1800: F, t1877: F, t1684: F, t1735: F, t1732: F, t1738: F, t532: F, t533: F, t529: F, t1792: F, t534: F, t452: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1879 = F::cast_from(19.489173774580152_f64) * t1877 * t1800;
    let t1880 = F::cast_from(0.9421211958699838_f64) * t1684;
    let t1882 = F::cast_from(0.3140403986233279_f64) * t1735;
    let t1884 = t1880 - F::cast_from(0.9421211958699838_f64) * t1732 + t1882 + F::cast_from(0.9421211958699838_f64) * t1738;
    let t1887 = F::cast_from(1.0_f64) / t533 / t532;
    let t1888 = t529 * t1887;
    let t1891 = t1884 * t534 - t1888 * t1792 / F::cast_from(2.0_f64);
    let t1892 = t529 * t529;
    let t1893 = F::cast_from(1.0_f64) / t532;
    let t1895 = -t1892 * t1893 + F::cast_from(1.0_f64);
    let t1896 = F::cast_from(1.0_f64) / t1895;
    let t1897 = t1891 * t1896;
    let t1898 = t1897 * t452;
    (t1879, t1880, t1882, t1884, t1887, t1888, t1892, t1893, t1895, t1896, t1897, t1898)
}
