//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1029/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1029(t31518: f64, t652: f64, t671: f64, t8533: f64, t9348: f64, t23831: f64, t7042: f64, t23858: f64, t8607: f64, t26161: f64, t31775: f64, t92200: f64) -> (f64, f64, f64, f64, f64) {
    let t115672 = 4.0_f64 * t652 * t31518 * t671;
    let t115674 = 2.0_f64 * t9348 * t8533;
    let t115676 = 2.0_f64 * t7042 * t23831;
    let t115678 = 2.0_f64 * t8607 * t23858;
    let t115681 = 4.0_f64 * t26161 * t92200 * t31775;
    (t115672, t115674, t115676, t115678, t115681)
}
