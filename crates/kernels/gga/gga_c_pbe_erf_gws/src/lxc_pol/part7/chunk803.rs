//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 803/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk803<F: Float>(t3138: F, t6652: F, t2253: F, t2277: F, t6592: F, t6597: F, t6600: F, t6604: F, t6607: F, t6614: F, t6618: F, t6623: F, t6625: F, t6628: F, t6633: F, t6637: F, t6640: F, t6650: F) -> (F, F) {
    let t6654 = t3138 * t6652 / F::new(16.0);
    let t6655 = -t6592 - t6597 - t2277 * t6600 / F::new(768.0) - t6604 + t6607 + t6614 + t6618 + t6623 - t6625 - F::new(7.0) / F::new(96.0) * t6628 - t2253 * t6633 / F::new(128.0) + t6637 * t6640 / F::new(256.0) + t6650 + t6654;
    (t6654, t6655)
}
