//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1032/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1032(t47075: f64, t14453: f64, t501: f64, t605: f64, t47077: f64, t42537: f64, t42540: f64, t42544: f64, t42547: f64, t42551: f64, t42570: f64, t42573: f64, t42580: f64, t42582: f64, t42588: f64, t42591: f64, t46852: f64, t46859: f64, t46862: f64, t46865: f64, t46871: f64, t46877: f64) -> (f64, f64, f64, f64) {
    let t50931 = 2.0_f64 * t47075;
    let t50932 = t14453 * t501;
    let t50933 = t50932 * t605;
    let t50934 = 4.0_f64 * t47077;
    let t50939 = -0.12646669615856066489e-1_f64 * t46852 + t42537 + t42540 + t42544 - t42547 - t42551 - t42570 - t42573 - t42580 + 0.23712505529730124666e-2_f64 * t46859 - 0.71137516589190373998e-2_f64 * t46862 + 0.47425011059460249332e-2_f64 * t46865 + t46871 + t42582 - t42588 - t42591 - t46877;
    (t50931, t50933, t50934, t50939)
}
