//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 991/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk991<F: Float>(t3178: F, t337: F, t814: F, t2147: F, t2120: F, t3180: F, t6253: F, t3106: F, t360: F, t2306: F, t3074: F, t2138: F) -> (F, F, F) {
    let t8873 = t337 * t3178 * t814;
    let t8874 = t2147 * t8873;
    let t8876 = t2120 * t8874 / F::cast_from(48.0_f64);
    let t8878 = t6253 * t3180 / F::cast_from(48.0_f64);
    let t8879 = t3106 * t360;
    let t8880 = t2306 * t8879;
    let t8881 = t3074 * t8880;
    let t8883 = t8881 * t2138 / F::cast_from(48.0_f64);
    (t8876, t8878, t8883)
}
