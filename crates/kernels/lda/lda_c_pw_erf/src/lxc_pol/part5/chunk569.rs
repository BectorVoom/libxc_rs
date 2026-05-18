//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 569/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk569<F: Float>(t299: F, t411: F, t732: F, t3257: F, t120: F, t1652: F, t19: F, t1657: F, t3216: F, t1653: F, t2061: F, t1953: F, t432: F) -> (F, F, F, F, F, F) {
    let t3259 = t732 * t299 * t411;
    let t3260 = t3257 * t3259;
    let t3267 = t1652 * t120 * t19;
    let t3268 = t3267 * t3259;
    let t3276 = t1657 * t3216;
    let t3280 = F::new(1.2991222222222223) * t1653 * t2061;
    let t3282 = F::new(0.7617244444444444) * t432 * t1953;
    (t3260, t3267, t3268, t3276, t3280, t3282)
}
