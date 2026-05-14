//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 865/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk865<F: Float>(t8306: F, t8310: F, t8328: F, t8341: F, t8313: F, t8316: F, t8324: F, t8326: F, t8339: F, t8346: F, t8348: F, t8355: F, t8370: F, t8374: F, t8353: F, t8358: F, t8376: F, t8379: F, t8382: F, t8386: F, t8388: F, t8390: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11430 = 3.8973666666666666 * t8306;
    let t11431 = 0.48717083333333333 * t8310;
    let t11436 = 0.9743416666666667 * t8328;
    let t11437 = 2.923025 * t8341;
    let t11439 = -t11430 + t11431 - 2.0 / 3.0 * t8313 + t8316 / 6.0 - 1.46904 * t8324 + 0.73452 * t8326 + t11436 - t8339 + t11437 + t8346 + 14.6904 * t8348;
    let t11441 = 4.5469277777777775 * t8355;
    let t11443 = 3.8973666666666666 * t8370;
    let t11444 = 1.9486833333333333 * t8374;
    let t11451 = 6.85552 * t8353 + t11441 + 14.0 / 9.0 * t8358 + t11443 - t11444 + 11.75232 * t8376 + 2.0 * t8379 + 2.0 * t8382 + 5.87616 * t8386 + 5.87616 * t8388 - 2.93808 * t8390;
    (t11430, t11431, t11436, t11437, t11439, t11441, t11443, t11444, t11451)
}
