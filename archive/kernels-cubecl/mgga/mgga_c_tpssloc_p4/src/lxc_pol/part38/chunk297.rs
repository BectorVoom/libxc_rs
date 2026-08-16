//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 297/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk297<F: Float>(t884: F, t908: F, t136: F, t886: F, t897: F, t899: F, t902: F, t907: F) -> (F, F, F) {
    let t909 = t908 * t884;
    let t910 = t136 * t909;
    let t912 = F::cast_from(0.1898925e1_f64) * t897 - t899 - F::cast_from(0.29896666666666666667e0_f64) * t886 + F::cast_from(0.3071625e0_f64) * t902 - t907 - F::cast_from(0.82156666666666666667e-1_f64) * t910;
    (t909, t910, t912)
}
