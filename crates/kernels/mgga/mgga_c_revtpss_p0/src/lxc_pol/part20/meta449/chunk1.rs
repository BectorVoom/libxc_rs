//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1713/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1713<F: Float>(t4003: F, t46477: F, t1437: F, t4004: F, t4114: F, t4118: F, t46518: F, t46520: F, t46522: F, t46526: F, t46532: F, t46536: F, t46540: F, t46542: F, t820: F, t9891: F, t9899: F) -> (F, F) {
    let t46547 = t46477 * t4003;
    let t46551 = t46518 - F::cast_from(0.78059524315062264152e-1_f64) * t46520 + F::cast_from(0.79025390195226139183e1_f64) * t820 * t46522 * t4004 + F::cast_from(0.44178176337912614788e-3_f64) * t46526 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t4118 * t9891 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t1437 * t46532 - F::cast_from(0.43902994552903410657e-1_f64) * t46536 + F::cast_from(0.21951497276451705328e-1_f64) * t46540 - F::cast_from(0.43902994552903410657e-1_f64) * t46542 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t4118 * t9899 + F::cast_from(0.92196288561097162379e1_f64) * t820 * t4114 * t46547;
    (t46547, t46551)
}
