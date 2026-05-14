//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1075/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1075<F: Float>(t44629: F, t44672: F, t2157: F, t3717: F, t11478: F, t2170: F, t3138: F, t13347: F, t2168: F, t13334: F, t3131: F, t3139: F, t44695: F, t11600: F, t11925: F, t2312: F, t2343: F, t2345: F, t3257: F, t36626: F, t3803: F, t3814: F, t44282: F, t44763: F, t816: F) -> (F, F, F, F, F, F, F, F, F) {
    let t49371 = 7.0 / 24.0 * t44629;
    let t49372 = 7.0 / 12.0 * t44672;
    let t49374 = t2157 * t3717;
    let t49378 = t3138 * t2170 * t11478 * t49374 / 4.0;
    let t49382 = t2168 * t2170 * t11478 * t13347 / 8.0;
    let t49387 = t3138 * t3139 * t3131 * t2157 * t13334 / 12.0;
    let t49388 = 7.0 / 4.0 * t44695;
    let t49399 = 3.0 / 8.0 * t11600 * t11925;
    let t49401 = t49371 + t49372 + 119.0 / 144.0 * t36626 - t49378 + t49382 + t49387 + t49388 + t2343 * t2345 * t44282 * t3814 / 96.0 - t2312 * t3257 * t3803 * t816 * t3717 / 32.0 + t49399 + 7.0 / 24.0 * t44763;
    (t49371, t49372, t49374, t49378, t49382, t49387, t49388, t49399, t49401)
}
