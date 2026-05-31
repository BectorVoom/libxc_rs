//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1041/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1041<F: Float>(t346: F, t9847: F, t1114: F, t2124: F, t274: F, t3028: F, t2255: F, t3258: F, t11640: F, t11646: F, t11650: F, t11652: F, t11656: F, t11665: F, t2277: F, t3247: F, t8927: F, t9447: F, t9457: F, t9464: F, t9474: F) -> (F, F, F) {
    let t11667 = t9847 * t346;
    let t11668 = t1114 * t11667;
    let t11670 = t11668 * t2124 / F::cast_from(96.0_f64);
    let t11671 = t274 * t3028;
    let t11673 = t2255 * t3258 * t11671;
    let t11676 = F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t3247 * t11640 - t8927 + t11646 - t11650 - t3247 * t11652 / F::cast_from(64.0_f64) + t2277 * t11656 / F::cast_from(384.0_f64) + t11665 + t9447 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t9457 - t11670 - t9464 + t9474 - t2277 * t11673 / F::cast_from(1536.0_f64);
    (t11670, t11673, t11676)
}
