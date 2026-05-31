//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 654/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk654<F: Float>(t1109: F, t369: F, t1130: F, t810: F, t2494: F, t339: F, t2178: F, t2181: F, t3028: F, t340: F, t870: F, t871: F) -> (F, F, F, F) {
    let t3154 = t1109 * t369;
    let t3159 = t1130 * t810;
    let t3162 = t339 * t2494;
    let t3165 = -t3028 * t339 * t340 + F::cast_from(3.0_f64) * t1130 * t2178 - F::cast_from(12.0_f64) * t2181 * t3159 + F::cast_from(3.0_f64) * t3154 * t871 + F::cast_from(3.0_f64) * t3162 * t870;
    (t3154, t3159, t3162, t3165)
}
