//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1109/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1109(t12263: f64, t12275: f64, t1919: f64, t454: f64, t11461: f64, t11913: f64, t12227: f64, t12237: f64, t12241: f64, t12244: f64, t1803: f64, t1901: f64, t1904: f64, t1934: f64, t2752: f64, t2811: f64, t2914: f64, t2924: f64, t455: f64, t6679: f64, t6694: f64, t6704: f64, t7513: f64, t7526: f64, t7528: f64, t7530: f64, t7533: f64) -> f64 {
    let t12276 = t12263 + t12275;
    let t12277 = t12276 * t1919;
    let t12278 = t454 * t12277;
    let t12289 = 1.1846959580306418_f64 * t12227 + 2.2140749178833072_f64 * t6679 * t2752 + 2.2140749178833072_f64 * t1803 * t11461 + t7526 - t6704 * t2811 + 0.04115066352984959_f64 * t7513 * t2924 + 0.04115066352984959_f64 * t1904 * t12237 + 4.937333717448355_f64 * t12241 * t455 + 4.937333717448355_f64 * t12244 * t455 - 4.937333717448355_f64 * t6694 * t2752 - 4.937333717448355_f64 * t1934 * t11461 - 0.04115066352984959_f64 * t1904 * t12278 - 22.07984838129906_f64 * t11913 - 18.635258017632964_f64 * t1901 * t11461 - 0.04115066352984959_f64 * t7513 * t2914 - 1.8805371096875316_f64 * t7528 + 19.489173774580152_f64 * t7530 - 19.489173774580152_f64 * t7533;
    t12289
}
