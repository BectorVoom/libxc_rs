//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1023/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1023<F: Float>(t20599: F, t6678: F, t2158: F, t810: F, t2407: F, t858: F, t8988: F, t6274: F, t6684: F, t824: F, t19993: F, t2210: F, t884: F, t20571: F, t20576: F, t20582: F, t20584: F, t20588: F, t20593: F, t20594: F, t2255: F, t2277: F, t6276: F, t6665: F) -> (F, F, F, F) {
    let t20601 = t6678 * t20599 / 16.0;
    let t20602 = t2158 * t810;
    let t20604 = t2407 * t858 * t20602;
    let t20606 = t8988 * t20604 / 2.0;
    let t20607 = t6684 * t6274;
    let t20608 = t824 * t20602;
    let t20615 = t884 * t2210 * t858 * t19993 / 4.0;
    let t20616 = t2277 * t2255 * t20571 * t6665 / 256.0 + 7.0 / 24.0 * t20576 - t20582 + t20584 - t20588 - t20593 - 7.0 / 144.0 * t20594 + t20601 - t20606 - 3.0 / 16.0 * t20607 * t6276 * t20608 + t20615;
    (t20601, t20606, t20615, t20616)
}
