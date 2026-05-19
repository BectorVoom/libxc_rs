//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1407/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1407<F: Float>(t21990: F, t21994: F, t21997: F, t21999: F, t22004: F, t22009: F, t22010: F, t22012: F, t22014: F, t22019: F, t22024: F, t22030: F, t22033: F, t22038: F, t22042: F, t22045: F, t22046: F, t30373: F) -> F {
    let t30439 = t21990 - t21994 - F::new(24.0) * t21997 - F::cast_from(0.20508037716432813316e4_f64) * t21999 - t30373 - F::new(240.0) * t22004 + t22009 + F::cast_from(0.70178683471615754484e1_f64) * t22010 - F::cast_from(0.11393789434848516922e-2_f64) * t22012 - F::cast_from(0.10389515463408878255e3_f64) * t22014 - t22019 - t22024 - t22030 - t22033 - t22038 - t22042 - t22045 + F::cast_from(0.24415263074675393405e-3_f64) * t22046;
    t30439
}
