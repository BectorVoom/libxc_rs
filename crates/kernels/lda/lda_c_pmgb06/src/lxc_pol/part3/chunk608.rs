//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 608/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk608<F: Float>(t3382: F, t518: F, t166: F, t161: F, t1400: F, t187: F, t186: F, t395: F, t184: F, t1403: F, t1410: F, t474: F, t955: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3383 = t518 * t3382;
    let t3384 = t166 * t3383;
    let t3386 = t161 * t3384 / F::new(30.0);
    let t3387 = t1400 * t187;
    let t3389 = t395 * t186;
    let t3391 = F::cast_from(0.0011033703703703704_f64) * t184 * t3389;
    let t3392 = t1403 * t187;
    let t3395 = F::new(4.0) * t1410 * t187;
    let t3396 = t955 * t474;
    (t3383, t3384, t3386, t3387, t3389, t3391, t3392, t3395, t3396)
}
