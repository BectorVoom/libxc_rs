//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1392/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1392(t52715: f64, t55633: f64, t55634: f64, t57213: f64, t57216: f64, t57219: f64, t57223: f64, t57225: f64, t57227: f64, t57229: f64, t57231: f64, t57233: f64, t57235: f64) -> f64 {
    let t58776 = -t52715 + 7.0_f64 / 288.0_f64 * t57213 + t55633 - t55634 + t57216 / 48.0_f64 - t57219 / 24.0_f64 - t57223 / 48.0_f64 + t57225 / 32.0_f64 + t57227 / 192.0_f64 + t57229 / 24.0_f64 - t57231 / 192.0_f64 + t57233 / 24.0_f64 + 5.0_f64 / 96.0_f64 * t57235;
    t58776
}
