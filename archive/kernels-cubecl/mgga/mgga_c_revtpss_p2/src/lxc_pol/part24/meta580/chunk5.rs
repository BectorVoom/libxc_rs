//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1798/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1798<F: Float>(t487: F, t90132: F, t1285: F, t1287: F, t17853: F, t17854: F, t17958: F, t20956: F, t24919: F, t24928: F, t24948: F, t24986: F, t24998: F, t25002: F, t25005: F, t45786: F, t45787: F, t5436: F, t5478: F, t59550: F, t59674: F, t59788: F, t6622: F, t6714: F, t69637: F, t82859: F, t90870: F) -> (F, F) {
    let t91536 = t487 * t90132;
    let t91544 = -F::cast_from(0.26341796731742046395e1_f64) * t5478 * t82859 * t24998 + F::cast_from(0.26341796731742046395e1_f64) * t59550 * t24948 - F::cast_from(0.23707617058567841754e2_f64) * t17853 * t20956 * t17854 * t6622 + F::cast_from(0.79025390195226139183e1_f64) * t5436 * t24919 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t487 * t90870 * t1287 - F::cast_from(0.15805078039045227836e2_f64) * t59788 * t25002 + F::cast_from(0.79025390195226139183e1_f64) * t59674 * t25005 - F::cast_from(0.79025390195226139183e1_f64) * t17958 * t24986 + F::cast_from(0.92196288561097162379e1_f64) * t45786 * t91536 * t45787 + F::cast_from(0.79025390195226139183e1_f64) * t69637 * t6714 + F::cast_from(0.79025390195226139183e1_f64) * t5436 * t24928;
    (t91536, t91544)
}
