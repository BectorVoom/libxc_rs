//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1101/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1101(t10848: f64, t3415: f64, t40498: f64, t40527: f64, t40547: f64, t40563: f64, t47586: f64, t47587: f64, t47616: f64, t47617: f64, t47618: f64, t47622: f64, t47626: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47628 = 16.0_f64 / 15.0_f64 * t10848 * t3415;
    let t47629 = 32.0_f64 / 15.0_f64 * t40498;
    let t47630 = 64.0_f64 / 45.0_f64 * t40527;
    let t47631 = 16.0_f64 / 45.0_f64 * t40547;
    let t47632 = 16.0_f64 / 15.0_f64 * t40563;
    let t47633 = t47586 - t47587 + t47616 - t47617 - t47618 + t47622 - t47626 - t47628 + t47629 + t47630 - t47631 - t47632;
    (t47628, t47629, t47630, t47631, t47632, t47633)
}
