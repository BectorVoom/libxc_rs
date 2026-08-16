//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 627/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk627(t1643: f64, t643: f64, t8654: f64, t2265: f64, t631: f64, t8621: f64, t8626: f64, t8630: f64, t8636: f64, t8641: f64, t8643: f64, t8645: f64, t8647: f64, t8650: f64, t8652: f64) -> (f64, f64) {
    let t8655 = t1643 * t643;
    let t8656 = t8654 * t8655;
    let t8659 = 6.0_f64 * t631 * t8621 - 9.0_f64 / 2.0_f64 * t631 * t8626 + t631 * t8630 / 6.0_f64 + 2.0_f64 / 27.0_f64 * t631 * t8636 + 5.0_f64 / 9.0_f64 * t8641 - t8643 / 3.0_f64 - t8645 / 9.0_f64 + 3.0_f64 * t8647 + t631 * t8650 - t2265 * t8652 - t2265 * t8656 / 3.0_f64;
    (t8656, t8659)
}
