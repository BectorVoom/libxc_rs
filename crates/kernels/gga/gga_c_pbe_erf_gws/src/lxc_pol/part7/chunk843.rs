//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 843/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk843<F: Float>(t2344: F, t904: F, t4383: F, t6158: F, t2157: F, t3222: F, t1185: F, t346: F, t825: F, t38: F, t368: F, t4340: F, param_a_c: F) -> (F, F, F, F, F, F, F, F) {
    let t9665 = t2344 * t904;
    let t11374 = t6158 * t4383;
    let t11540 = t2157 * param_a_c;
    let t11541 = t11540 * t3222;
    let t12076 = t346 * t825 * t1185;
    let t15651 = t38 * t38;
    let t15652 = F::new(1.0) / t15651;
    let t16191 = t368 * t368;
    let t16192 = F::new(1.0) / t16191;
    let t16329 = F::cast_from(0.12654485932329694421e2_f64) * t4340;
    (t9665, t11374, t11541, t12076, t15651, t15652, t16192, t16329)
}
