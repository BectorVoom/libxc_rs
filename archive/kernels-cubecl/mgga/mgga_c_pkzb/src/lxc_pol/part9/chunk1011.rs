//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1011/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1011<F: Float>(t3070: F, t851: F, t2197: F, t1185: F, t2234: F, t2198: F, t3073: F, t6142: F, t2242: F, t3069: F, t2240: F, t1184: F, t6201: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8189 = t3070 * t851;
    let t8191 = F::cast_from(4.0_f64) * t2197 * t8189;
    let t8192 = t1185 * t2234;
    let t8194 = F::cast_from(2.0_f64) * t2197 * t8192;
    let t8195 = t3073 * t2198;
    let t8197 = F::cast_from(0.96491876992155210402e2_f64) * t6142 * t8195;
    let t8198 = t3069 * t2242;
    let t8199 = t8198 * t851;
    let t8201 = F::cast_from(0.32163958997385070134e2_f64) * t2240 * t8199;
    let t8202 = t3073 * t2234;
    let t8204 = F::cast_from(0.16081979498692535067e2_f64) * t2240 * t8202;
    let t8205 = t1184 * t6201;
    (t8189, t8191, t8192, t8194, t8195, t8197, t8198, t8199, t8201, t8202, t8204, t8205)
}
