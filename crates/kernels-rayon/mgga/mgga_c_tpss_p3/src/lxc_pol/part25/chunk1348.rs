//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1348/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1348(t3255: f64, t6419: f64, t5380: f64, t5918: f64, t62375: f64, t67138: f64, t69489: f64, t69491: f64, t69493: f64, t69495: f64, t69497: f64, t69499: f64, t69501: f64, t69503: f64, t69505: f64, t69507: f64) -> (f64, f64, f64) {
    let t71725 = t3255 * t6419;
    let t71748 = t5918 * t5380;
    let t71776 = t67138 + t69489 / 96.0_f64 - 5.0_f64 / 96.0_f64 * t69491 - t62375 - t69493 / 48.0_f64 - t69495 / 128.0_f64 + t69497 / 128.0_f64 + t69499 / 192.0_f64 - t69501 / 768.0_f64 - t69503 / 96.0_f64 - t69505 / 768.0_f64 - 5.0_f64 / 192.0_f64 * t69507;
    (t71725, t71748, t71776)
}
