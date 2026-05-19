//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 21/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk21<F: Float>(t41: F, t21: F, t17: F, t28: F) -> (F, F, F, F) {
    let t43 = F::new(1.0) - F::new(1.0) / t41;
    let t45 = t21 * t43 + F::new(1.0);
    let t46 = F::ln(t45);
    let t48 = -F::new(0.285764e-1) * t17 + F::new(0.285764e-1) * t46;
    let t49 = t28 - F::new(1.0);
    (t43, t45, t48, t49)
}
