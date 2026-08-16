//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2817/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2817(t231: f64, t2782: f64, t2783: f64, t76131: f64, t40325: f64, t836: f64, t14972: f64, t18616: f64, t18681: f64, t39624: f64, t4424: f64, t4514: f64, t4526: f64, t51320: f64, t51355: f64, t5978: f64, t62644: f64, t62649: f64, t62651: f64, t62653: f64, t76169: f64, t820: f64) -> (f64, f64) {
    let t76182 = t2782 * t2783 * t76131 * t231;
    let t76194 = t40325 * t836;
    let t76198 = 0.16463622957338778996e-1_f64 * t62644 - 0.21951497276451705328e-1_f64 * t62649 - 0.21951497276451705328e-1_f64 * t62651 + 0.19514881078765566037e-2_f64 * t62653 - 0.11044544084478153697e-3_f64 * t39624 + 0.16463622957338778997e-1_f64 * t76182 - 0.39512695097613069592e1_f64 * t4514 * t18681 * t4424 - 0.19514881078765566038e-2_f64 * t51355 - 0.19756347548806534796e1_f64 * t820 * t4526 * t18616 - 0.19756347548806534796e1_f64 * t820 * t14972 * t5978 + 0.15805078039045227836e2_f64 * t51320 * t76169 * t76194;
    (t76194, t76198)
}
