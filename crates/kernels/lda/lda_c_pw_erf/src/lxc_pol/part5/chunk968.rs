//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 968/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk968<F: Float>(t13517: F, t1184: F, t2177: F, t519: F, t521: F, t4729: F, t511: F, t2061: F, t830: F, t11845: F, t2062: F, t1351: F, t588: F) -> (F, F, F, F, F, F) {
    let t13518 = F::new(8.0) / F::new(135.0) * t13517;
    let t13523 = t519 * t1184 * t521 * t2177;
    let t13550 = t511 * t4729;
    let t13551 = F::new(4.0) / F::new(45.0) * t13550;
    let t13562 = t2061 * t830;
    let t13564 = t11845 * t2062;
    let t13631 = t588 * t1351;
    (t13518, t13523, t13551, t13562, t13564, t13631)
}
