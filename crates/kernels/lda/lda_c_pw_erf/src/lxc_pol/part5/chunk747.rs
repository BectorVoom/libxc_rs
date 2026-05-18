//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 747/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk747<F: Float>(t5176: F, t4029: F, t2405: F, t509: F, t184: F, t199: F, t2407: F, t515: F, t2523: F, t331: F, t2517: F, t2520: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6785 = F::new(16.0) / F::new(45.0) * t5176;
    let t6786 = F::new(4.0) / F::new(135.0) * t4029;
    let t6787 = t2405 * t509;
    let t6788 = t6787 * t184;
    let t6790 = F::new(4.0) / F::new(15.0) * t6788 * t199;
    let t6791 = t2407 * t515;
    let t6792 = F::new(8.0) / F::new(45.0) * t6791;
    let t6793 = t331 * t2523;
    let t6795 = t331 * t2517;
    let t6797 = t331 * t2520;
    (t6785, t6786, t6787, t6788, t6790, t6791, t6792, t6793, t6795, t6797)
}
