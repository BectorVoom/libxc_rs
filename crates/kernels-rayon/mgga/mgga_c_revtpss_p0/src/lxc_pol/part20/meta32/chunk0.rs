//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 239/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk239(t112: f64, t625: f64, t111: f64, t43: f64, t605: f64, tau0: f64) -> (f64, f64, f64, f64, f64) {
    let t653 = t625 * t112 / 3.0_f64;
    let t654 = t111 * t111;
    let t655 = 1.0_f64 / t654;
    let t656 = tau0 * t43;
    let t658 = t605 / 2.0_f64;
    (t653, t654, t655, t656, t658)
}
