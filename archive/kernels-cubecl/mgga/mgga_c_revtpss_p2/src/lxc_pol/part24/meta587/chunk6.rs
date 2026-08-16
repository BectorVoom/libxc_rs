//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1830/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1830<F: Float>(t91882: F, t91927: F, t92081: F, t92123: F, t92136: F, t92168: F, t92195: F, t92216: F, t213: F, t225: F, t23043: F, t46359: F, t46368: F, t46385: F, t46388: F, t47764: F, t47772: F, t47781: F, t47786: F, t47802: F, t561: F, t5715: F, t73587: F, t73593: F, t73623: F, t85475: F) -> (F, F) {
    let t92219 = t91882 + t91927 + t92081 + t92123 + t92136 + t92168 + t92195 + t92216;
    let t92229 = F::cast_from(0.78548797528808629095e-3_f64) * t47764 - F::cast_from(0.78059524315062264152e-1_f64) * t73587 + F::cast_from(0.7805952431506226415e-2_f64) * t73593 + F::cast_from(0.44178176337912614788e-3_f64) * t47772 - F::cast_from(0.78548797528808629095e-3_f64) * t47781 - F::cast_from(0.1040793657534163522e-1_f64) * t47786 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t92219 * t225 * t561 + t46359 - t46368 + F::cast_from(0.68293547082294194357e-1_f64) * t47802 - F::cast_from(0.26341796731742046395e1_f64) * t5715 * t23043 + F::cast_from(0.43902994552903410657e-1_f64) * t73623 - F::cast_from(0.39029762157531132076e-1_f64) * t85475 - t46385 - t46388;
    (t92219, t92229)
}
