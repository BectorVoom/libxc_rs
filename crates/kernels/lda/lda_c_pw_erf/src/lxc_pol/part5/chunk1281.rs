//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1281/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1281<F: Float>(t22922: F, t3974: F, t4522: F, t593: F, t14089: F, t12765: F, t1325: F, t2471: F, t542: F, t784: F, t2098: F, t5289: F, t6431: F) -> (F, F, F, F) {
    let t22944 = F::new(8.0) / F::new(9.0) * t3974 * t4522 * t22922 * t593;
    let t22945 = F::new(16.0) / F::new(135.0) * t14089;
    let t22950 = F::new(24.0) / F::new(5.0) * t1325 * t12765 * t2471 * t784 * t542;
    let t22954 = F::new(16.0) / F::new(5.0) * t1325 * t5289 * t6431 * t2098;
    (t22944, t22945, t22950, t22954)
}
