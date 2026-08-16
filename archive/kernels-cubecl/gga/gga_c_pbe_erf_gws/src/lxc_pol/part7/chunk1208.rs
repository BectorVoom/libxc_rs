//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1208/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1208<F: Float>(t2273: F, t6717: F, t2339: F, t6416: F, t6667: F, t2119: F, t2124: F, t6106: F, t20803: F, t21447: F, t21452: F, t21455: F, t21456: F, t21462: F, t2266: F, t2271: F, t3247: F, t6105: F, t902: F, t904: F, t905: F, t916: F, t9665: F) -> (F, F) {
    let t21463 = t6717 * t2273;
    let t21465 = t6717 * t2339;
    let t21474 = t6416 * t6667;
    let t21478 = t6106 * t2119 * t2124 / F::cast_from(32.0_f64);
    let t21479 = F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t2266 * t916 * t904 * t21447 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t21452 - t21455 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t21456 + t21462 + F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t21463 + F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t21465 + t902 * t905 * t6105 * t2271 / F::cast_from(512.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t3247 * t9665 * t20803 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t21474 - t21478;
    (t21478, t21479)
}
