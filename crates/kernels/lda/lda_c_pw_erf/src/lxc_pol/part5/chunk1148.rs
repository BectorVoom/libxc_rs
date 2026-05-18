//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1148/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1148<F: Float>(t17645: F, t2001: F, t3974: F, t16624: F, t16626: F, t16633: F, t16648: F, t16650: F, t16652: F, t16702: F, t16709: F, t34: F, t6335: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21128 = F::new(16.0) / F::new(15.0) * t3974 * t17645 * t2001;
    let t21129 = F::new(32.0) / F::new(45.0) * t16624;
    let t21130 = F::new(32.0) / F::new(45.0) * t16626;
    let t21131 = F::new(16.0) / F::new(27.0) * t16633;
    let t21132 = F::new(32.0) / F::new(45.0) * t16648;
    let t21133 = F::new(64.0) / F::new(45.0) * t16650;
    let t21134 = F::new(32.0) / F::new(27.0) * t16652;
    let t21135 = F::new(16.0) / F::new(45.0) * t16702;
    let t21136 = F::new(16.0) / F::new(45.0) * t16709;
    let t21137 = t6335 * t34;
    (t21128, t21129, t21130, t21131, t21132, t21133, t21134, t21135, t21136, t21137)
}
