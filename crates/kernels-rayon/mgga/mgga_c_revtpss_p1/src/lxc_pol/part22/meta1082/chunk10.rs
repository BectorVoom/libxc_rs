//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3911/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3911(t10139: f64, t136: f64, t2457: f64, t6844: f64, t14145: f64, t14171: f64, t1882: f64, t2482: f64, t10069: f64, t22361: f64, t22365: f64, t13805: f64, t14193: f64, t22005: f64, t22009: f64, t4057: f64, t46563: f64, t46570: f64, t46572: f64, t49238: f64, t49242: f64, t5675: f64, t5745: f64, t5755: f64, t74973: f64, t74982: f64) -> f64 {
    let t75128 = t10139 * t6844 * t136 * t2457;
    let t75141 = t2482 * t14171 * t1882 * t14145;
    let t75145 = t10069 * t22361;
    let t75147 = t10069 * t22365;
    let t75155 = -0.11565819519348392139e-2_f64 * t75128 + 0.26341796731742046394e1_f64 * t5745 * t74973 * t5675 - 0.65854491829355115987e0_f64 * t5755 * t22005 * t4057 - 0.14634331517634470219e-1_f64 * t46563 + 0.34146773541147097178e-1_f64 * t46570 + 0.13009920719177044025e-2_f64 * t46572 + 0.78059524315062264149e-1_f64 * t75141 + 0.2601984143835408805e-2_f64 * t49238 + 0.10975748638225852664e-1_f64 * t49242 + 0.14634331517634470219e-1_f64 * t75145 - 0.14634331517634470219e-1_f64 * t75147 + 0.79025390195226139182e1_f64 * t5745 * t74982 * t5675 - 0.39512695097613069591e1_f64 * t14193 * t22009 * t13805;
    t75155
}
