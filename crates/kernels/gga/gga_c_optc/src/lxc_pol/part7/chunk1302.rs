//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1302/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1302<F: Float>(t26252: F, t26258: F, t26262: F, t26265: F, t26268: F, t26271: F, t26326: F, t26328: F, t26330: F, t26332: F, t26347: F, t26351: F, t26354: F, t26358: F) -> F {
    let t26360 = F::cast_from(0.44291358024691358024e0_f64) * t26252 + F::cast_from(0.39862222222222222223e1_f64) * t26258 + t26262 + t26265 + F::cast_from(0.1151859375e0_f64) * t26268 + F::cast_from(0.46074375e0_f64) * t26271 + F::cast_from(0.1898925e1_f64) * t26347 - F::cast_from(0.79724444444444444446e0_f64) * t26326 - F::cast_from(0.5314962962962962963e0_f64) * t26328 - F::cast_from(0.43816888888888888888e0_f64) * t26351 + F::cast_from(0.43816888888888888889e0_f64) * t26354 + F::cast_from(0.15944888888888888889e1_f64) * t26330 + F::cast_from(0.12401580246913580247e1_f64) * t26332 + F::cast_from(0.97370864197530864199e0_f64) * t26358;
    t26360
}
