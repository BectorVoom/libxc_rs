//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3298/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3298<F: Float>(t4321: F, t4534: F, t689: F, t213: F, t225: F, t257: F, t40994: F, t40998: F, t40999: F, t41003: F, t41004: F, t41006: F, t41014: F, t41021: F, t41029: F, t41034: F, t50245: F, t50248: F, t50253: F, t50259: F, t61430: F, t61437: F, t61441: F, t61448: F, t62509: F) -> F {
    let t62516 = t689 * t4321 * t4534;
    let t62518 = F::cast_from(0.39029762157531132074e-1_f64) * t61430 + F::cast_from(0.2601984143835408805e-2_f64) * t50245 + F::cast_from(0.73171657588172351096e-2_f64) * t40994 + F::cast_from(0.22089088168956307394e-3_f64) * t50248 - t40998 + F::cast_from(0.39029762157531132075e-1_f64) * t61437 - F::cast_from(0.14634331517634470219e-1_f64) * t40999 - F::cast_from(0.1040793657534163522e0_f64) * t50253 - F::cast_from(0.19514881078765566038e-1_f64) * t61441 - t41003 + F::cast_from(0.34146773541147097178e-1_f64) * t41004 + F::cast_from(0.14634331517634470219e-1_f64) * t41006 + F::cast_from(0.78059524315062264152e-1_f64) * t50259 + F::cast_from(0.23131639038696784278e-2_f64) * t41014 - F::cast_from(0.23131639038696784278e-2_f64) * t41021 + F::cast_from(0.73171657588172351096e-2_f64) * t61448 + F::cast_from(0.11565819519348392139e-2_f64) * t41029 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t62509 * t225 * t257 + F::cast_from(0.52039682876708176101e-2_f64) * t41034 + F::cast_from(0.21951497276451705328e-1_f64) * t62516;
    t62518
}
