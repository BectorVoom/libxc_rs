//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1473/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1473(t122595: f64, t122597: f64, t122598: f64, t122599: f64, t122600: f64, t122602: f64, t122603: f64, t122604: f64, t122605: f64, t122606: f64, t122608: f64, t26906: f64, t5361: f64, t7040: f64, t8103: f64, t8690: f64, t8840: f64) -> f64 {
    let t124994 = 3.0_f64 * t26906 * t8690 + t5361 * t8840 - t7040 * t8103 + t122595 - 2.0_f64 * t122597 - 2.0_f64 * t122598 - 2.0_f64 * t122599 - 2.0_f64 * t122600 - 2.0_f64 * t122602 - 2.0_f64 * t122603 - 2.0_f64 * t122604 - 2.0_f64 * t122605 - 2.0_f64 * t122606 - 2.0_f64 * t122608;
    t124994
}
