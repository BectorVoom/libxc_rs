//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1072/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1072<F: Float>(t6416: F, t6667: F, t2119: F, t2124: F, t6106: F, t20803: F, t21447: F, t21452: F, t21455: F, t21456: F, t21462: F, t21463: F, t21465: F, t2266: F, t2271: F, t3247: F, t6105: F, t902: F, t904: F, t905: F, t916: F, t9665: F) -> (F, F) {
    let t21474 = t6416 * t6667;
    let t21478 = t6106 * t2119 * t2124 / 32.0;
    let t21479 = 3.0 / 512.0 * t2266 * t916 * t904 * t21447 + 7.0 / 576.0 * t21452 - t21455 - 7.0 / 288.0 * t21456 + t21462 + 119.0 / 1152.0 * t21463 + 119.0 / 1152.0 * t21465 + t902 * t905 * t6105 * t2271 / 512.0 - 3.0 / 32.0 * t3247 * t9665 * t20803 - 7.0 / 288.0 * t21474 - t21478;
    (t21478, t21479)
}
