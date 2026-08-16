//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 507/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk507(t1919: f64, t2912: f64, t454: f64, t1942: f64, t2777: f64, t452: f64, t2149: f64, t514: f64, t1905: f64, t1783: f64, t1803: f64, t1808: f64, t1823: f64, t1859: f64, t1901: f64, t1904: f64, t1934: f64, t2007: f64, t2752: f64, t2811: f64, t2872: f64, t2877: f64, t2890: f64, t2903: f64, t455: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2913 = t2912 * t1919;
    let t2914 = t454 * t2913;
    let t2919 = t2777 * t1942;
    let t2920 = t2919 * t452;
    let t2923 = t514 * t2149;
    let t2924 = t1905 * t2923;
    let t2927 = -t2007 * t2811 + 1.8805371096875316_f64 * t2872 * t455 - 19.489173774580152_f64 * t1808 * t2752 + 19.489173774580152_f64 * t2877 * t455 - 1.8805371096875316_f64 * t1859 * t2752 + 3.7610742193750633_f64 * t1823 * t2752 - 1.7770439370459628_f64 * t1783 * t2890 + 2.2140749178833072_f64 * t1803 * t2752 - 2.2140749178833072_f64 * t2903 * t455 - 18.635258017632964_f64 * t1901 * t2752 - 0.04115066352984959_f64 * t1904 * t2914 - 4.937333717448355_f64 * t1934 * t2752 + 4.937333717448355_f64 * t2920 * t455 + 0.04115066352984959_f64 * t1904 * t2924;
    (t2913, t2914, t2919, t2920, t2923, t2924, t2927)
}
