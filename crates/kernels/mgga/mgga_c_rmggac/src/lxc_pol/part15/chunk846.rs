//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 846/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk846<F: Float>(t41789: F, t1550: F, t41548: F, t1978: F, t7228: F, t8511: F, t1982: F, t7428: F, t16156: F, t9198: F, t388: F, t575: F, t7933: F, t7934: F) -> (F, F, F, F, F, F) {
    let t41790 = F::new(0.15965655602485078085e0) * t41789;
    let t41791 = t1550 * t41548;
    let t41792 = F::new(0.15965655602485078085e0) * t41791;
    let t41799 = t8511 * t7228 * t1978;
    let t41811 = t8511 * t7428 * t1982;
    let t41812 = F::new(0.19863479950205658386e-4) * t41811;
    let t41813 = t16156 * t9198;
    let t41817 = t7933 * t7934 * t388 * t575;
    (t41790, t41792, t41799, t41812, t41813, t41817)
}
