//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1966/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1966(t92578: f64, t98610: f64, t98612: f64, t98614: f64, t98616: f64, t98618: f64, t98620: f64, t98622: f64, t98624: f64, t98626: f64, t98629: f64, t98631: f64, t98633: f64, t98635: f64, t98637: f64, t98639: f64, t98642: f64) -> f64 {
    let t101398 = t98610 / 96.0_f64 + t98612 / 96.0_f64 + t98614 / 96.0_f64 + t98616 / 192.0_f64 - 7.0_f64 / 144.0_f64 * t98618 + t98620 / 128.0_f64 + t98622 / 384.0_f64 - t92578 - t98624 / 768.0_f64 - t98626 / 128.0_f64 + t98629 / 192.0_f64 - t98631 / 96.0_f64 + t98633 / 192.0_f64 + t98635 / 192.0_f64 - t98637 / 384.0_f64 - t98639 / 768.0_f64 - 0.33913115119077928317e-1_f64 * t98642;
    t101398
}
