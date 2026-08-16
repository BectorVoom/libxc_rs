//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3298/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3298(t4321: f64, t4534: f64, t689: f64, t213: f64, t225: f64, t257: f64, t40994: f64, t40998: f64, t40999: f64, t41003: f64, t41004: f64, t41006: f64, t41014: f64, t41021: f64, t41029: f64, t41034: f64, t50245: f64, t50248: f64, t50253: f64, t50259: f64, t61430: f64, t61437: f64, t61441: f64, t61448: f64, t62509: f64) -> f64 {
    let t62516 = t689 * t4321 * t4534;
    let t62518 = 0.39029762157531132074e-1_f64 * t61430 + 0.2601984143835408805e-2_f64 * t50245 + 0.73171657588172351096e-2_f64 * t40994 + 0.22089088168956307394e-3_f64 * t50248 - t40998 + 0.39029762157531132075e-1_f64 * t61437 - 0.14634331517634470219e-1_f64 * t40999 - 0.1040793657534163522e0_f64 * t50253 - 0.19514881078765566038e-1_f64 * t61441 - t41003 + 0.34146773541147097178e-1_f64 * t41004 + 0.14634331517634470219e-1_f64 * t41006 + 0.78059524315062264152e-1_f64 * t50259 + 0.23131639038696784278e-2_f64 * t41014 - 0.23131639038696784278e-2_f64 * t41021 + 0.73171657588172351096e-2_f64 * t61448 + 0.11565819519348392139e-2_f64 * t41029 + 0.65854491829355115987e0_f64 * t213 * t62509 * t225 * t257 + 0.52039682876708176101e-2_f64 * t41034 + 0.21951497276451705328e-1_f64 * t62516;
    t62518
}
