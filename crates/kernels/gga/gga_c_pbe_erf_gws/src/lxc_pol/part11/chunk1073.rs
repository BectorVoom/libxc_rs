//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1073/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1073<F: Float>(t1820: F, t2559: F, t30455: F, t3342: F, t12822: F, t2612: F, t12767: F, t30630: F, t10629: F, t3407: F, t1017: F, t40558: F, t7703: F) -> (F, F, F, F, F) {
    let t47297 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1820 * t2559 * t30455 * t3342;
    let t47299 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t2612 * t12822;
    let t47301 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t30630 * t12767;
    let t47303 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t10629 * t3407;
    let t47307 = F::cast_from(32.0_f64) / F::cast_from(5.0_f64) * t1820 * t7703 * t40558 * t1017;
    (t47297, t47299, t47301, t47303, t47307)
}
