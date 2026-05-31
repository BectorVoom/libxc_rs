//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 852/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk852<F: Float>(t219: F, t7209: F, t1642: F, t34: F, t422: F, t639: F, t1621: F, t1791: F, t1044: F, t617: F, t661: F, t1620: F) -> (F, F, F) {
    let t7210 = t7209 * t219;
    let t7211 = t1642 * t34;
    let t7212 = t7211 * t422;
    let t7213 = t7210 * t7212;
    let t7215 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t639 * t7213;
    let t7216 = t1621 * t1791;
    let t7217 = t1044 * t617;
    let t7218 = t7217 * t661;
    let t7219 = t7216 * t7218;
    let t7221 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1620 * t7219;
    (t7212, t7215, t7221)
}
