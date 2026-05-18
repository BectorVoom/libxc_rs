//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1007/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1007<F: Float>(t493: F, t7426: F, t7438: F, t7446: F, t7452: F, t7456: F, t7459: F, t7463: F, t7466: F, t7496: F, t7498: F, t7503: F, t7506: F, t7509: F, t7512: F, t7518: F, t9369: F) -> F {
    let t9390 = t7496 + F::new(0.21687162600603479684e-1) * t7498 + t7426 - t7503 + F::new(40.0) * t7506 + t7438 + t7446 - t7452 + F::new(0.19751673498613801407e-1) * t9369 * t493 + t7509 + t7456 - t7459 - t7463 - F::new(0.18311447306006545054e-3) * t7512 - t7466 - t7518;
    t9390
}
