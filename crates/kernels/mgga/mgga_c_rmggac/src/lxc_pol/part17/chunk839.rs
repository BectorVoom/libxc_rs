//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 839/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk839<F: Float>(t2100: F, t41056: F, t2103: F, t41036: F, t2118: F, t35959: F, t3839: F, t25640: F, t40998: F, t3851: F, t39696: F, t5259: F) -> (F, F, F, F, F, F, F) {
    let t41377 = t2100 * t41056;
    let t41378 = F::new(0.18183107769496894486e-1) * t41377;
    let t41379 = t2103 * t41036;
    let t41380 = F::new(0.24244143692662525982e-1) * t41379;
    let t41381 = t2118 * t41036;
    let t41400 = t3839 * t35959;
    let t41404 = t25640 * t40998;
    let t41407 = t3851 * t35959;
    let t41438 = t5259 * t39696;
    (t41378, t41380, t41381, t41400, t41404, t41407, t41438)
}
