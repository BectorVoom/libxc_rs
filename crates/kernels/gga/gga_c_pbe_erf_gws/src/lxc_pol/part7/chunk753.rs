//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 753/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk753<F: Float>(t2118: F, t6638: F, t6276: F, t339: F, t911: F, t824: F, t822: F, t2157: F, t6177: F, t337: F, t2121: F, t2135: F, t3139: F, t6360: F, t3138: F, t2253: F, t2277: F, t6592: F, t6597: F, t6600: F, t6604: F, t6607: F, t6614: F, t6618: F, t6623: F, t6625: F, t6628: F, t6633: F, t6637: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6639 = t2118 * t6638;
    let t6640 = t6276 * t6639;
    let t6643 = t339 * t911;
    let t6644 = t824 * t6643;
    let t6645 = t822 * t6644;
    let t6646 = t6177 * t2157;
    let t6647 = t337 * t6646;
    let t6648 = t2121 * t6647;
    let t6650 = t6645 * t6648 / 16.0;
    let t6652 = t3139 * t2135 * t6360;
    let t6654 = t3138 * t6652 / 16.0;
    let t6655 = -t6592 - t6597 - t2277 * t6600 / 768.0 - t6604 + t6607 + t6614 + t6618 + t6623 - t6625 - 7.0 / 96.0 * t6628 - t2253 * t6633 / 128.0 + t6637 * t6640 / 256.0 + t6650 + t6654;
    (t6639, t6640, t6643, t6644, t6645, t6647, t6648, t6650, t6652, t6654, t6655)
}
