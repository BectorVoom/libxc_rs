//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 579/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk579<F: Float>(t2901: F, t2903: F, t2905: F, t2907: F, t2915: F, t2921: F, t2926: F, t2930: F, t2935: F, t2941: F, t3380: F, t3369: F, t518: F, t166: F, t161: F, t1400: F, t187: F) -> (F, F, F, F, F) {
    let t3381 = -0.21595 * t2930 + 0.21595 * t2935 - 0.07198333333333333 * t2905 + 0.14396666666666666 * t2921 - 0.07198333333333333 * t2926 - 0.047988888888888886 * t2901 + 0.035991666666666665 * t2907 + 0.023994444444444443 * t2903 - 0.03999074074074074 * t2915 - 0.035991666666666665 * t2941 - t3380;
    let t3382 = t3369 + t3381;
    let t3383 = t518 * t3382;
    let t3384 = t166 * t3383;
    let t3386 = t161 * t3384 / 30.0;
    let t3387 = t1400 * t187;
    (t3382, t3383, t3384, t3386, t3387)
}
