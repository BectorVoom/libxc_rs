//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1013/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1013<F: Float>(t1445: F, t41413: F, t41418: F, t41422: F, t41428: F, t42993: F, t44064: F, t44065: F, t44069: F, t44070: F, t44074: F, t44076: F, t44079: F, t44083: F, t44085: F, t44089: F, t44092: F, t44093: F, t44097: F, t44099: F, t44106: F, t807: F) -> F {
    let t44108 = -t44064 + F::cast_from(0.21450293971110256002e1_f64) * t44065 + t44069 - F::cast_from(0.59584149919750711116e-1_f64) * t44070 - t44074 + F::cast_from(0.13803453343411469884e2_f64) * t44076 + t44079 - t44083 - t44085 - t44089 - t44092 - F::cast_from(0.13803453343411469884e2_f64) * t44093 - t44097 - t44099 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t1445 * t42993 + F::cast_from(0.38342925953920749676e0_f64) * t41413 + F::cast_from(0.38342925953920749676e0_f64) * t41418 - F::cast_from(0.85206502119823888169e-1_f64) * t41422 + t44106 + F::cast_from(0.51123901271894332901e0_f64) * t41428;
    t44108
}
