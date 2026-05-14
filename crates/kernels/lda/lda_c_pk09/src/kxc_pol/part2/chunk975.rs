//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 975/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk975<F: Float>(t12263: F, t12275: F, t1919: F, t454: F, t11461: F, t11913: F, t12227: F, t12237: F, t12241: F, t12244: F, t1803: F, t1901: F, t1904: F, t1934: F, t2752: F, t2811: F, t2914: F, t2924: F, t455: F, t6679: F, t6694: F, t6704: F, t7513: F, t7526: F, t7528: F, t7530: F, t7533: F) -> (F,) {
    let t12276 = t12263 + t12275;
    let t12277 = t12276 * t1919;
    let t12278 = t454 * t12277;
    let t12289 = 1.1846959580306418 * t12227 + 2.2140749178833072 * t6679 * t2752 + 2.2140749178833072 * t1803 * t11461 + t7526 - t6704 * t2811 + 0.04115066352984959 * t7513 * t2924 + 0.04115066352984959 * t1904 * t12237 + 4.937333717448355 * t12241 * t455 + 4.937333717448355 * t12244 * t455 - 4.937333717448355 * t6694 * t2752 - 4.937333717448355 * t1934 * t11461 - 0.04115066352984959 * t1904 * t12278 - 22.07984838129906 * t11913 - 18.635258017632964 * t1901 * t11461 - 0.04115066352984959 * t7513 * t2914 - 1.8805371096875316 * t7528 + 19.489173774580152 * t7530 - 19.489173774580152 * t7533;
    (t12289,)
}
