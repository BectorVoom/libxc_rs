//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 785/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk785<F: Float>(t2522: F, t299: F, t169: F, t242: F, t2994: F, t700: F, t784: F, t991: F, t2998: F, t1086: F, t1383: F, t331: F, t551: F, t553: F, t1371: F, t2948: F) -> (F, F, F, F, F, F, F, F) {
    let t8352 = t299 * t2522;
    let t8355 = 0.1061188859155979109e0 * t169 * t8352 * t242;
    let t8357 = t169 * t2994 * t700;
    let t8361 = t784 * t991;
    let t8363 = t169 * t8361 * t242;
    let t8371 = 0.63671331549358746542e-1 * t169 * t2998 * t700;
    let t8373 = t169 * t1086 * t1383;
    let t8382 = t331 * t2522;
    let t8385 = 0.39507780657818961764e-2 * t8382 * t551 * t553;
    let t8387 = t2948 * t1371 * t553;
    (t8355, t8357, t8361, t8363, t8371, t8373, t8385, t8387)
}
