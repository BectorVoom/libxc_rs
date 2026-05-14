//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 855/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk855<F: Float>(t169: F, t2994: F, t700: F, t784: F, t991: F, t242: F, t171: F, t7908: F, t2998: F, t1086: F, t1383: F, t145: F, t5700: F, t5723: F, t5726: F, t5730: F, t5732: F, t5735: F, t8038: F) -> (F, F, F) {
    let t8357 = t169 * t2994 * t700;
    let t8361 = t784 * t991;
    let t8363 = t169 * t8361 * t242;
    let t8365 = t171 * t7908;
    let t8371 = 0.63671331549358746542e-1 * t169 * t2998 * t700;
    let t8373 = t169 * t1086 * t1383;
    let t8379 = -0.1066501354843587606e0 * t5735 - 0.14149184788746388121e0 * t8363 - 0.31835665774679373271e-1 * t169 * t8365 * t242 - t8371 - 0.31835665774679373271e-1 * t8373 + 0.533250677421793803e-1 * t145 * t8038 - 0.31835665774679373271e-1 * t5723 - 0.63671331549358746542e-1 * t5726 - t5730 + t5700 - t5732;
    (t8357, t8361, t8379)
}
