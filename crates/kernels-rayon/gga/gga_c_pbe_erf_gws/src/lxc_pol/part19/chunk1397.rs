//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1397/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1397(t15532: f64, t840: f64, t52560: f64, t55850: f64, t55851: f64, t55863: f64, t55884: f64, t57584: f64, t57593: f64, t57595: f64, t57598: f64, t57602: f64, t57605: f64, t57608: f64, t57614: f64, t57626: f64, t57635: f64) -> f64 {
    let t58875 = t840 * t15532;
    let t58883 = -t55850 + t55851 + t57584 / 384.0_f64 + t57593 / 384.0_f64 - 35.0_f64 / 432.0_f64 * t52560 + t57595 / 12.0_f64 - t57598 / 24.0_f64 + t55863 + 7.0_f64 / 144.0_f64 * t58875 - t57602 / 192.0_f64 - t57605 / 24.0_f64 - t57608 / 48.0_f64 + t57614 / 8.0_f64 - t57626 / 384.0_f64 + t55884 - t57635 / 768.0_f64;
    t58883
}
