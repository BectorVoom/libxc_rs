//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1098/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1098<F: Float>(t4386: F, t6795: F, t892: F, t4383: F, t4408: F, t822: F, t4482: F, t859: F, t19599: F, t19605: F, t19608: F, t19612: F, t19618: F, t19623: F, t19627: F, t19634: F, t19639: F, t19643: F, t19647: F, t19652: F, t2362: F, t4385: F, t4484: F, t6164: F, t6793: F, t827: F) -> (F, F) {
    let t19655 = t4386 * t892 * t6795;
    let t19658 = t4408 * t4383;
    let t19659 = t822 * t19658;
    let t19663 = t859 * t892 * t4482;
    let t19666 = t4385 * t19599 / F::cast_from(24.0_f64) + t6793 * t19605 / F::cast_from(4.0_f64) - t19608 * t19612 / F::cast_from(12.0_f64) + t4385 * t19618 / F::cast_from(12.0_f64) - t827 * t19623 / F::cast_from(8.0_f64) + t822 * t19627 * t6164 / F::cast_from(16.0_f64) - t827 * t19634 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19639 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t827 * t19643 - t822 * t19647 * t2362 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19652 + t6793 * t19655 / F::cast_from(2.0_f64) + t19659 * t4484 / F::cast_from(12.0_f64) + t4385 * t19663 / F::cast_from(8.0_f64);
    (t19659, t19666)
}
