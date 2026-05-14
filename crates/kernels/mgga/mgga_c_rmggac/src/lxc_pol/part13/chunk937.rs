//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 937/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk937<F: Float>(t9343: F, t942: F, t1356: F, t2211: F, t27120: F, t27146: F, t40262: F, t40266: F, t40270: F, t40274: F, t40279: F, t40283: F, t40287: F, t40291: F, t40295: F, t40297: F, t40302: F, t40307: F, t40314: F, t40319: F, t40324: F, t739: F, t8041: F) -> (F,) {
    let t43366 = 0.4726e1 * t942 * t9343;
    let t43370 = -0.23948483403727617128e0 * t1356 * t8041 * t27120 + 0.23948483403727617128e0 * t739 * t2211 * t27146 - 0.79453919800822633545e-4 * t40262 + 0.638468998399467591e-4 * t40266 + 0.5107751987195740728e-4 * t40270 - 0.10215503974391481456e-3 * t40274 - 0.1702583995731913576e-4 * t40279 + 0.15323255961587222184e-3 * t40283 - 0.5107751987195740728e-4 * t40287 + 0.15323255961587222184e-3 * t40291 + 0.17961362552795712846e0 * t40295 + 0.5987120850931904282e-1 * t40297 + 0.35922725105591425692e0 * t40302 + 0.17961362552795712846e0 * t40307 - t43366 + 0.212822999466489197e-4 * t40314 - 0.638468998399467591e-4 * t40319 + 0.638468998399467591e-4 * t40324;
    (t43370,)
}
