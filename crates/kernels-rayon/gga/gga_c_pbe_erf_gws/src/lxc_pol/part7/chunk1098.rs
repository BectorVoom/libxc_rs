//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1098/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1098(t4386: f64, t6795: f64, t892: f64, t4383: f64, t4408: f64, t822: f64, t4482: f64, t859: f64, t19599: f64, t19605: f64, t19608: f64, t19612: f64, t19618: f64, t19623: f64, t19627: f64, t19634: f64, t19639: f64, t19643: f64, t19647: f64, t19652: f64, t2362: f64, t4385: f64, t4484: f64, t6164: f64, t6793: f64, t827: f64) -> (f64, f64) {
    let t19655 = t4386 * t892 * t6795;
    let t19658 = t4408 * t4383;
    let t19659 = t822 * t19658;
    let t19663 = t859 * t892 * t4482;
    let t19666 = t4385 * t19599 / 24.0_f64 + t6793 * t19605 / 4.0_f64 - t19608 * t19612 / 12.0_f64 + t4385 * t19618 / 12.0_f64 - t827 * t19623 / 8.0_f64 + t822 * t19627 * t6164 / 16.0_f64 - t827 * t19634 / 8.0_f64 + 7.0_f64 / 24.0_f64 * t19639 + 3.0_f64 / 4.0_f64 * t827 * t19643 - t822 * t19647 * t2362 / 32.0_f64 - 7.0_f64 / 24.0_f64 * t19652 + t6793 * t19655 / 2.0_f64 + t19659 * t4484 / 12.0_f64 + t4385 * t19663 / 8.0_f64;
    (t19659, t19666)
}
