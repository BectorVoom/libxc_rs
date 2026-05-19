//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1076/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1076<F: Float>(t34927: F, t34931: F, t37266: F, t39119: F, t39122: F, t45222: F, t45226: F, t45234: F, t45240: F, t45242: F, t45244: F, t45249: F, t45254: F, t45259: F, t45264: F, t45266: F, t45272: F) -> F {
    let t48394 = F::cast_from(0.11974241701863808564e0_f64) * t45222 + F::cast_from(0.35922725105591425692e0_f64) * t45226 - t37266 + F::cast_from(0.68400385060046895e-6_f64) * t34927 + F::cast_from(0.68400385060046895e-6_f64) * t34931 + F::cast_from(0.40992351065071538964e-3_f64) * t39119 + F::cast_from(0.85129199786595678799e-5_f64) * t45234 + F::cast_from(0.13242319966803772257e-3_f64) * t39122 - F::cast_from(0.1702583995731913576e-4_f64) * t45240 - F::cast_from(0.39726959900411316773e-4_f64) * t45242 - F::cast_from(0.11918087970123395032e-3_f64) * t45244 - F::cast_from(0.212822999466489197e-4_f64) * t45249 + F::cast_from(0.638468998399467591e-4_f64) * t45254 - F::cast_from(0.638468998399467591e-4_f64) * t45259 - F::cast_from(0.425645998932978394e-4_f64) * t45264 + F::cast_from(0.1702583995731913576e-4_f64) * t45266 + F::cast_from(0.85129199786595678799e-5_f64) * t45272;
    t48394
}
