//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1221/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1221<F: Float>(t44695: F, t11600: F, t11925: F, t2312: F, t2343: F, t2345: F, t3257: F, t36626: F, t3717: F, t3803: F, t3814: F, t44282: F, t44763: F, t49371: F, t49372: F, t49378: F, t49382: F, t49387: F, t816: F) -> (F, F, F) {
    let t49388 = F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t44695;
    let t49399 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t11600 * t11925;
    let t49401 = t49371 + t49372 + F::cast_from(119.0_f64) / F::cast_from(144.0_f64) * t36626 - t49378 + t49382 + t49387 + t49388 + t2343 * t2345 * t44282 * t3814 / F::cast_from(96.0_f64) - t2312 * t3257 * t3803 * t816 * t3717 / F::cast_from(32.0_f64) + t49399 + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t44763;
    (t49388, t49399, t49401)
}
