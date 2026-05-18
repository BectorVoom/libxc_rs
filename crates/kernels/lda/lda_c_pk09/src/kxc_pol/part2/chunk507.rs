//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 507/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk507<F: Float>(t1919: F, t2912: F, t454: F, t1942: F, t2777: F, t452: F, t2149: F, t514: F, t1905: F, t1783: F, t1803: F, t1808: F, t1823: F, t1859: F, t1901: F, t1904: F, t1934: F, t2007: F, t2752: F, t2811: F, t2872: F, t2877: F, t2890: F, t2903: F, t455: F) -> (F, F, F, F, F, F, F) {
    let t2913 = t2912 * t1919;
    let t2914 = t454 * t2913;
    let t2919 = t2777 * t1942;
    let t2920 = t2919 * t452;
    let t2923 = t514 * t2149;
    let t2924 = t1905 * t2923;
    let t2927 = -t2007 * t2811 + F::new(1.8805371096875316) * t2872 * t455 - F::new(19.489173774580152) * t1808 * t2752 + F::new(19.489173774580152) * t2877 * t455 - F::new(1.8805371096875316) * t1859 * t2752 + F::new(3.7610742193750633) * t1823 * t2752 - F::new(1.7770439370459628) * t1783 * t2890 + F::new(2.2140749178833072) * t1803 * t2752 - F::new(2.2140749178833072) * t2903 * t455 - F::new(18.635258017632964) * t1901 * t2752 - F::new(0.04115066352984959) * t1904 * t2914 - F::new(4.937333717448355) * t1934 * t2752 + F::new(4.937333717448355) * t2920 * t455 + F::new(0.04115066352984959) * t1904 * t2924;
    (t2913, t2914, t2919, t2920, t2923, t2924, t2927)
}
