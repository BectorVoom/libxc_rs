//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1798/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1798(t487: f64, t90132: f64, t1285: f64, t1287: f64, t17853: f64, t17854: f64, t17958: f64, t20956: f64, t24919: f64, t24928: f64, t24948: f64, t24986: f64, t24998: f64, t25002: f64, t25005: f64, t45786: f64, t45787: f64, t5436: f64, t5478: f64, t59550: f64, t59674: f64, t59788: f64, t6622: f64, t6714: f64, t69637: f64, t82859: f64, t90870: f64) -> (f64, f64) {
    let t91536 = t487 * t90132;
    let t91544 = -0.26341796731742046395e1_f64 * t5478 * t82859 * t24998 + 0.26341796731742046395e1_f64 * t59550 * t24948 - 0.23707617058567841754e2_f64 * t17853 * t20956 * t17854 * t6622 + 0.79025390195226139183e1_f64 * t5436 * t24919 + 0.65854491829355115987e0_f64 * t1285 * t487 * t90870 * t1287 - 0.15805078039045227836e2_f64 * t59788 * t25002 + 0.79025390195226139183e1_f64 * t59674 * t25005 - 0.79025390195226139183e1_f64 * t17958 * t24986 + 0.92196288561097162379e1_f64 * t45786 * t91536 * t45787 + 0.79025390195226139183e1_f64 * t69637 * t6714 + 0.79025390195226139183e1_f64 * t5436 * t24928;
    (t91536, t91544)
}
