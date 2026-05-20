//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3639/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3639<F: Float>(t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t56230: F, t56236: F, t68389: F, t68393: F, t68397: F, t68399: F, t68402: F, t68464: F) -> F {
    let t68870 = -F::cast_from(0.39862222222222222222e0_f64) * t56187 - F::cast_from(0.11958666666666666667e1_f64) * t56189 + F::cast_from(0.26574814814814814814e0_f64) * t56209 + F::cast_from(0.13287407407407407407e0_f64) * t56212 + F::cast_from(0.79724444444444444443e0_f64) * t56214 - F::cast_from(0.22145679012345679012e0_f64) * t56216 + F::cast_from(0.5314962962962962963e0_f64) * t56228 - F::cast_from(0.19931111111111111111e0_f64) * t56230 - F::cast_from(0.62007901234567901235e0_f64) * t56236 - F::cast_from(0.19931111111111111111e0_f64) * t68389 + F::cast_from(0.29896666666666666667e0_f64) * t68393 - F::cast_from(0.39862222222222222222e0_f64) * t68397 + F::cast_from(0.26574814814814814815e0_f64) * t68399 + F::cast_from(0.36514074074074074075e-1_f64) * t68402 + F::new(0.1898925e1) * t68464;
    t68870
}
