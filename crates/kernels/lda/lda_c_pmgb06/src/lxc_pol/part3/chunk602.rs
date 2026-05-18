//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 602/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk602<F: Float>(t3306: F, t186: F, t409: F, t55: F, t543: F, t1400: F, t27: F, t545: F, t1403: F, t3271: F, t3273: F, t3275: F, t3278: F, t3282: F, t3287: F, t3289: F, t3294: F, t3297: F, t3299: F, t3302: F, t3305: F) -> (F, F, F, F, F, F, F, F) {
    let t3307 = t3306 / F::new(45.0);
    let t3309 = t55 * t409 * t186;
    let t3311 = F::new(0.09618703433213194) * t543 * t3309;
    let t3312 = t1400 * t27;
    let t3313 = t3312 * t545;
    let t3315 = t1403 * t27;
    let t3316 = t3315 * t545;
    let t3318 = t3271 + t3273 + t3275 + t3278 + t3282 + t3287 + t3289 + t3294 + t3297 + t3299 + t3302 + t3305 - t3307 - t3311 + F::new(0.3246312408709453) * t3313 + F::new(0.6492624817418906) * t3316;
    (t3307, t3309, t3311, t3312, t3313, t3315, t3316, t3318)
}
