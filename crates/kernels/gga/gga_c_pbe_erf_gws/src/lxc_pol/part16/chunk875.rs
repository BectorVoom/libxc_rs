//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 875/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk875<F: Float>(t1031: F, t1781: F, t184: F, t221: F, t1631: F, t2612: F, t2740: F, t586: F, t1624: F, t2636: F, t5018: F, t1820: F) -> (F, F, F, F, F) {
    let t7521 = t1781 * t1031;
    let t7522 = t7521 * t184;
    let t7524 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t7522 * t221;
    let t7526 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t2612 * t1631;
    let t7527 = t2740 * t586;
    let t7529 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7527 * t1624;
    let t7530 = t5018 * t2636;
    let t7532 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1820 * t7530;
    (t7524, t7526, t7527, t7529, t7532)
}
