//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1836/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1836<F: Float>(t22858: F, t22954: F, t47417: F, t49327: F, t49354: F, t49361: F, t5767: F, t820: F, t86575: F, t86582: F, t86586: F, t86597: F, t86604: F, t86608: F) -> F {
    let t92394 = F::cast_from(0.21951497276451705328e-1_f64) * t86575 + F::cast_from(0.65854491829355115985e-1_f64) * t86582 + F::cast_from(0.13170898365871023197e0_f64) * t86586 + F::cast_from(0.44178176337912614788e-3_f64) * t49354 + F::cast_from(0.78548797528808629095e-3_f64) * t49361 - t47417 - F::cast_from(0.21951497276451705328e-1_f64) * t86597 + F::cast_from(0.65854491829355115985e-1_f64) * t86604 + F::cast_from(0.21951497276451705328e-1_f64) * t86608 - F::cast_from(0.15805078039045227836e2_f64) * t820 * t49327 * t22858 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t5767 * t22954;
    t92394
}
