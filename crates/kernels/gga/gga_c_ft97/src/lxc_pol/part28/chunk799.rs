//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 799/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk799<F: Float>(t167: F, t34918: F, t574: F, t1391: F, t6615: F, t3578: F, t7407: F, t144: F, t7357: F, t925: F, t9144: F, t1053: F, t2179: F, t1017: F, t7400: F, t9439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t35181 = t574 * t167 * t34918;
    let t35185 = t574 * t1391 * t6615;
    let t35188 = t3578 * t7407;
    let t35189 = t144 * t35188;
    let t35192 = t7357 * t925;
    let t35193 = t9144 * t35192;
    let t35196 = t7407 * t1053;
    let t35197 = t2179 * t35196;
    let t35198 = t144 * t35197;
    let t35201 = t7400 * t1017;
    let t35203 = t574 * t2179 * t35201;
    let t35206 = t7400 * t1053;
    let t35207 = t9439 * t35206;
    (t35181, t35185, t35188, t35189, t35192, t35193, t35196, t35197, t35198, t35201, t35203, t35206, t35207)
}
