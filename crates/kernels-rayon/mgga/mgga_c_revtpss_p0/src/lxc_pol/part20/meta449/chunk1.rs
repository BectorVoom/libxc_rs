//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1713/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1713(t4003: f64, t46477: f64, t1437: f64, t4004: f64, t4114: f64, t4118: f64, t46518: f64, t46520: f64, t46522: f64, t46526: f64, t46532: f64, t46536: f64, t46540: f64, t46542: f64, t820: f64, t9891: f64, t9899: f64) -> (f64, f64) {
    let t46547 = t46477 * t4003;
    let t46551 = t46518 - 0.78059524315062264152e-1_f64 * t46520 + 0.79025390195226139183e1_f64 * t820 * t46522 * t4004 + 0.44178176337912614788e-3_f64 * t46526 - 0.26341796731742046395e1_f64 * t820 * t4118 * t9891 - 0.19756347548806534796e1_f64 * t820 * t1437 * t46532 - 0.43902994552903410657e-1_f64 * t46536 + 0.21951497276451705328e-1_f64 * t46540 - 0.43902994552903410657e-1_f64 * t46542 - 0.26341796731742046395e1_f64 * t820 * t4118 * t9899 + 0.92196288561097162379e1_f64 * t820 * t4114 * t46547;
    (t46547, t46551)
}
