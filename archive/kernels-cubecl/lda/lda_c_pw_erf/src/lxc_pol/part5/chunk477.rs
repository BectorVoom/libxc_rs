//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 477/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk477<F: Float>(t43: F, t2325: F, t2329: F, t47: F, t939: F, t743: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t2333 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t939 * t2325 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t2329);
    let t2334 = t743 * t743;
    (t2333, t2334)
}
