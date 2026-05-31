//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 932/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk932<F: Float>(t1648: F, t4893: F, t5134: F, t5312: F, t5530: F, t5533: F, t5536: F, t5540: F, t4982: F, t583: F, t17391: F, t17394: F, t17397: F, t17402: F) -> (F, F, F, F, F, F, F, F) {
    let t17404 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1648 * t4893;
    let t17406 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5312 * t5134;
    let t17408 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1648 * t5530;
    let t17410 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t1648 * t5533;
    let t17412 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1648 * t5536;
    let t17414 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t5312 * t5540;
    let t17415 = t4982 * t583;
    let t17416 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17415;
    let t17417 = t17391 + t17394 + t17397 + t17402 - t17404 + t17406 - t17408 - t17410 + t17412 + t17414 + t17416;
    (t17404, t17406, t17408, t17410, t17412, t17414, t17416, t17417)
}
