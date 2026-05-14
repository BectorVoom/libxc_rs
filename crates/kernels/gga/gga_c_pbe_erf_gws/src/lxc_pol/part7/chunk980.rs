//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 980/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk980<F: Float>(t19646: F, t825: F, t2382: F, t6154: F, t6778: F, t4386: F, t6795: F, t892: F, t4383: F, t4408: F, t822: F, t4482: F, t859: F, t19599: F, t19605: F, t19608: F, t19612: F, t19618: F, t19623: F, t19627: F, t19634: F, t19639: F, t19643: F, t2362: F, t4385: F, t4484: F, t6164: F, t6793: F, t827: F) -> (F, F) {
    let t19647 = t19646 * t825;
    let t19652 = t2382 * t6154 * t6778;
    let t19655 = t4386 * t892 * t6795;
    let t19658 = t4408 * t4383;
    let t19659 = t822 * t19658;
    let t19663 = t859 * t892 * t4482;
    let t19666 = t4385 * t19599 / 24.0 + t6793 * t19605 / 4.0 - t19608 * t19612 / 12.0 + t4385 * t19618 / 12.0 - t827 * t19623 / 8.0 + t822 * t19627 * t6164 / 16.0 - t827 * t19634 / 8.0 + 7.0 / 24.0 * t19639 + 3.0 / 4.0 * t827 * t19643 - t822 * t19647 * t2362 / 32.0 - 7.0 / 24.0 * t19652 + t6793 * t19655 / 2.0 + t19659 * t4484 / 12.0 + t4385 * t19663 / 8.0;
    (t19659, t19666)
}
