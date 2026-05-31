//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 910/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk910<F: Float>(t5217: F, t735: F, t5221: F, t211: F, t5098: F, t582: F, t1655: F, t4991: F, t587: F, t5351: F, t586: F, t645: F) -> (F, F, F, F) {
    let t17139 = t5217 * t735;
    let t17140 = t17139 * t5221;
    let t17141 = F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t17140;
    let t17143 = t211 * t582 * t5098;
    let t17144 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17143;
    let t17146 = t587 * t4991 * t1655;
    let t17147 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t17146;
    let t17148 = t5351 * t586;
    let t17150 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t17148 * t645;
    (t17141, t17144, t17147, t17150)
}
