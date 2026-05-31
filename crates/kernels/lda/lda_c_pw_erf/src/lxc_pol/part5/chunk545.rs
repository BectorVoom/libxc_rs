//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 545/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk545<F: Float>(t1738: F, t688: F, t1179: F, t684: F, t462: F, t34: F, t39: F) -> (F, F, F, F) {
    let t2842 = t1738 * t688;
    let t2847 = F::cast_from(0.05987117005127304_f64) * t684 * t1179;
    let t2848 = F::cast_from(12.0_f64) * t462;
    let t2849 = t34 * t39;
    (t2842, t2847, t2848, t2849)
}
