//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 803/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk803<F: Float>(t43: F, t1781: F, t2329: F, t2953: F, t47: F, t7354: F, t7360: F, t2334: F, t743: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t7364 = piecewise3::<F>(t44, F::new(0.0), -F::new(8.0) / F::new(27.0) * t2953 * t7354 + F::new(4.0) / F::new(3.0) * t1781 * t2329 + F::new(4.0) / F::new(3.0) * t47 * t7360);
    let t7365 = t2334 * t743;
    (t7364, t7365)
}
