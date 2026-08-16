//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1032/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1032<F: Float>(t47075: F, t14453: F, t501: F, t605: F, t47077: F, t42537: F, t42540: F, t42544: F, t42547: F, t42551: F, t42570: F, t42573: F, t42580: F, t42582: F, t42588: F, t42591: F, t46852: F, t46859: F, t46862: F, t46865: F, t46871: F, t46877: F) -> (F, F, F, F) {
    let t50931 = F::cast_from(2.0_f64) * t47075;
    let t50932 = t14453 * t501;
    let t50933 = t50932 * t605;
    let t50934 = F::cast_from(4.0_f64) * t47077;
    let t50939 = -F::cast_from(0.12646669615856066489e-1_f64) * t46852 + t42537 + t42540 + t42544 - t42547 - t42551 - t42570 - t42573 - t42580 + F::cast_from(0.23712505529730124666e-2_f64) * t46859 - F::cast_from(0.71137516589190373998e-2_f64) * t46862 + F::cast_from(0.47425011059460249332e-2_f64) * t46865 + t46871 + t42582 - t42588 - t42591 - t46877;
    (t50931, t50933, t50934, t50939)
}
