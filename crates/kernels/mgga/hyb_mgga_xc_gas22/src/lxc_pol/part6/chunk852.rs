//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 852/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk852<F: Float>(t6527: F, t6613: F, t2271: F, t819: F, t262: F) -> (F, F, F, F) {
    let t6691 = F::new(0.16068111111111111111e1) * t6527;
    let t6698 = F::new(0.46308888888888888888e0) * t6613;
    let t6709 = F::new(1.0) / t2271 / t819;
    let t6710 = t262 * t6709;
    (t6691, t6698, t6709, t6710)
}
