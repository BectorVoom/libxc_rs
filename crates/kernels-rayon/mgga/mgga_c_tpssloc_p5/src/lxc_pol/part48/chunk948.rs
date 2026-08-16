//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 948/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk948(t112984: f64, t112988: f64, t112990: f64, t112992: f64, t112995: f64, t114649: f64, t114655: f64, t114659: f64, t114663: f64, t114666: f64, t2633: f64, t2684: f64, t31394: f64, t812: f64, t829: f64) -> f64 {
    let t114668 = -2.0_f64 * t812 * t114649 * t829 - t812 * t31394 * t2684 + 2.0_f64 * t812 * t114655 * t2633 + 0.76763589786250567036e-1_f64 * t114659 - 0.16449340668482264365e-1_f64 * t114663 + 0.16449340668482264365e-1_f64 * t114666 + t112984 + t112988 + t112990 - t112992 + t112995;
    t114668
}
