//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2238/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2238(t87198: f64, t98610: f64, t98612: f64, t98614: f64, t98616: f64, t98618: f64, t98620: f64, t98622: f64, t98624: f64, t98626: f64, t98629: f64, t98631: f64, t98633: f64, t98635: f64, t98637: f64, t98639: f64, t98642: f64) -> f64 {
    let t98644 = t98610 / 192.0_f64 + t98612 / 192.0_f64 + t98614 / 192.0_f64 + t98616 / 384.0_f64 - 7.0_f64 / 288.0_f64 * t98618 + t98620 / 256.0_f64 + t98622 / 768.0_f64 - t87198 - t98624 / 1536.0_f64 - t98626 / 256.0_f64 + t98629 / 384.0_f64 - t98631 / 192.0_f64 + t98633 / 384.0_f64 + t98635 / 384.0_f64 - t98637 / 768.0_f64 - t98639 / 1536.0_f64 - 0.16956557559538964158e-1_f64 * t98642;
    t98644
}
