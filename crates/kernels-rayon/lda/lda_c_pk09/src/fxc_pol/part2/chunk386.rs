//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 386/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk386(t1800: f64, t1877: f64, t1684: f64, t1735: f64, t1732: f64, t1738: f64, t532: f64, t533: f64, t529: f64, t1792: f64, t534: f64, t452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1879 = 19.489173774580152_f64 * t1877 * t1800;
    let t1880 = 0.9421211958699838_f64 * t1684;
    let t1882 = 0.3140403986233279_f64 * t1735;
    let t1884 = t1880 - 0.9421211958699838_f64 * t1732 + t1882 + 0.9421211958699838_f64 * t1738;
    let t1887 = 1.0_f64 / t533 / t532;
    let t1888 = t529 * t1887;
    let t1891 = t1884 * t534 - t1888 * t1792 / 2.0_f64;
    let t1892 = t529 * t529;
    let t1893 = 1.0_f64 / t532;
    let t1895 = -t1892 * t1893 + 1.0_f64;
    let t1896 = 1.0_f64 / t1895;
    let t1897 = t1891 * t1896;
    let t1898 = t1897 * t452;
    (t1879, t1880, t1882, t1884, t1887, t1888, t1892, t1893, t1895, t1896, t1897, t1898)
}
