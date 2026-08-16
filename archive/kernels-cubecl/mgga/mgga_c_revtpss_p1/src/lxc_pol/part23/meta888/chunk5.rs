//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2817/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2817<F: Float>(t231: F, t2782: F, t2783: F, t76131: F, t40325: F, t836: F, t14972: F, t18616: F, t18681: F, t39624: F, t4424: F, t4514: F, t4526: F, t51320: F, t51355: F, t5978: F, t62644: F, t62649: F, t62651: F, t62653: F, t76169: F, t820: F) -> (F, F) {
    let t76182 = t2782 * t2783 * t76131 * t231;
    let t76194 = t40325 * t836;
    let t76198 = F::cast_from(0.16463622957338778996e-1_f64) * t62644 - F::cast_from(0.21951497276451705328e-1_f64) * t62649 - F::cast_from(0.21951497276451705328e-1_f64) * t62651 + F::cast_from(0.19514881078765566037e-2_f64) * t62653 - F::cast_from(0.11044544084478153697e-3_f64) * t39624 + F::cast_from(0.16463622957338778997e-1_f64) * t76182 - F::cast_from(0.39512695097613069592e1_f64) * t4514 * t18681 * t4424 - F::cast_from(0.19514881078765566038e-2_f64) * t51355 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t4526 * t18616 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t14972 * t5978 + F::cast_from(0.15805078039045227836e2_f64) * t51320 * t76169 * t76194;
    (t76194, t76198)
}
