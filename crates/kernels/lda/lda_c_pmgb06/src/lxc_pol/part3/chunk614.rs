//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 614/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk614<F: Float>(t170: F, t3457: F, t1602: F, t529: F, t166: F, t161: F, t3320: F, t3324: F, t3327: F, t3328: F, t3331: F, t3335: F, t3386: F, t3387: F, t3391: F, t3392: F, t3395: F, t3445: F, t3449: F, t3452: F, t3455: F) -> (F, F, F, F, F, F) {
    let t3458 = t170 * t3457;
    let t3459 = t1602 * t529;
    let t3460 = t3458 * t3459;
    let t3461 = t166 * t3460;
    let t3463 = t161 * t3461 / F::new(5.0);
    let t3464 = F::new(0.03354522822333102) * t3320 + t3324 + t3327 + F::new(0.21642082724729686) * t3328 + t3331 - t3335 - t3386 + F::new(4.0) * t3387 + t3391 + F::new(8.0) * t3392 + t3395 - t3445 - t3449 + t3452 - t3455 - t3463;
    (t3458, t3459, t3460, t3461, t3463, t3464)
}
