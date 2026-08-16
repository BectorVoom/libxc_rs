//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3911/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3911<F: Float>(t10139: F, t136: F, t2457: F, t6844: F, t14145: F, t14171: F, t1882: F, t2482: F, t10069: F, t22361: F, t22365: F, t13805: F, t14193: F, t22005: F, t22009: F, t4057: F, t46563: F, t46570: F, t46572: F, t49238: F, t49242: F, t5675: F, t5745: F, t5755: F, t74973: F, t74982: F) -> F {
    let t75128 = t10139 * t6844 * t136 * t2457;
    let t75141 = t2482 * t14171 * t1882 * t14145;
    let t75145 = t10069 * t22361;
    let t75147 = t10069 * t22365;
    let t75155 = -F::cast_from(0.11565819519348392139e-2_f64) * t75128 + F::cast_from(0.26341796731742046394e1_f64) * t5745 * t74973 * t5675 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t22005 * t4057 - F::cast_from(0.14634331517634470219e-1_f64) * t46563 + F::cast_from(0.34146773541147097178e-1_f64) * t46570 + F::cast_from(0.13009920719177044025e-2_f64) * t46572 + F::cast_from(0.78059524315062264149e-1_f64) * t75141 + F::cast_from(0.2601984143835408805e-2_f64) * t49238 + F::cast_from(0.10975748638225852664e-1_f64) * t49242 + F::cast_from(0.14634331517634470219e-1_f64) * t75145 - F::cast_from(0.14634331517634470219e-1_f64) * t75147 + F::cast_from(0.79025390195226139182e1_f64) * t5745 * t74982 * t5675 - F::cast_from(0.39512695097613069591e1_f64) * t14193 * t22009 * t13805;
    t75155
}
