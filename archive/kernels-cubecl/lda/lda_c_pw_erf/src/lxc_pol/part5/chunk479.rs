//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 479/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk479<F: Float>(t50: F, t2334: F, t2337: F, t52: F, t950: F, t2333: F, t59: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t2341 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t950 * t2334 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t2337);
    let t2343 = (t2333 + t2341) * t59;
    t2343
}
