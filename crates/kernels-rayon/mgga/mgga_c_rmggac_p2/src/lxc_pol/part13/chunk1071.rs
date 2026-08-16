//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1071/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1071(t9343: f64, t942: f64, t1356: f64, t2211: f64, t27120: f64, t27146: f64, t40262: f64, t40266: f64, t40270: f64, t40274: f64, t40279: f64, t40283: f64, t40287: f64, t40291: f64, t40295: f64, t40297: f64, t40302: f64, t40307: f64, t40314: f64, t40319: f64, t40324: f64, t739: f64, t8041: f64) -> f64 {
    let t43366 = 0.4726e1_f64 * t942 * t9343;
    let t43370 = -0.23948483403727617128e0_f64 * t1356 * t8041 * t27120 + 0.23948483403727617128e0_f64 * t739 * t2211 * t27146 - 0.79453919800822633545e-4_f64 * t40262 + 0.638468998399467591e-4_f64 * t40266 + 0.5107751987195740728e-4_f64 * t40270 - 0.10215503974391481456e-3_f64 * t40274 - 0.1702583995731913576e-4_f64 * t40279 + 0.15323255961587222184e-3_f64 * t40283 - 0.5107751987195740728e-4_f64 * t40287 + 0.15323255961587222184e-3_f64 * t40291 + 0.17961362552795712846e0_f64 * t40295 + 0.5987120850931904282e-1_f64 * t40297 + 0.35922725105591425692e0_f64 * t40302 + 0.17961362552795712846e0_f64 * t40307 - t43366 + 0.212822999466489197e-4_f64 * t40314 - 0.638468998399467591e-4_f64 * t40319 + 0.638468998399467591e-4_f64 * t40324;
    t43370
}
