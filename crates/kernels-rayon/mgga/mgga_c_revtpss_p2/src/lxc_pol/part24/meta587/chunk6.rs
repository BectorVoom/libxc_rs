//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1830/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1830(t91882: f64, t91927: f64, t92081: f64, t92123: f64, t92136: f64, t92168: f64, t92195: f64, t92216: f64, t213: f64, t225: f64, t23043: f64, t46359: f64, t46368: f64, t46385: f64, t46388: f64, t47764: f64, t47772: f64, t47781: f64, t47786: f64, t47802: f64, t561: f64, t5715: f64, t73587: f64, t73593: f64, t73623: f64, t85475: f64) -> (f64, f64) {
    let t92219 = t91882 + t91927 + t92081 + t92123 + t92136 + t92168 + t92195 + t92216;
    let t92229 = 0.78548797528808629095e-3_f64 * t47764 - 0.78059524315062264152e-1_f64 * t73587 + 0.7805952431506226415e-2_f64 * t73593 + 0.44178176337912614788e-3_f64 * t47772 - 0.78548797528808629095e-3_f64 * t47781 - 0.1040793657534163522e-1_f64 * t47786 + 0.65854491829355115987e0_f64 * t213 * t92219 * t225 * t561 + t46359 - t46368 + 0.68293547082294194357e-1_f64 * t47802 - 0.26341796731742046395e1_f64 * t5715 * t23043 + 0.43902994552903410657e-1_f64 * t73623 - 0.39029762157531132076e-1_f64 * t85475 - t46385 - t46388;
    (t92219, t92229)
}
