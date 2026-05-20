//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3186/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3186<F: Float>(t1222: F, t140: F, t24830: F, t12840: F, t17429: F, t17747: F, t20795: F, t21020: F, t21134: F, t24573: F, t3626: F, t5308: F, t5312: F, t5331: F, t5373: F, t59355: F, t6611: F, t70914: F, t70942: F, t81177: F, t81198: F, t81202: F, t82293: F) -> F {
    let t83699 = t1222 * t140 * t24830;
    let t83712 = F::cast_from(0.42874018118069736972e-3_f64) * t17429 * t24573 + F::cast_from(0.42874018118069736972e-3_f64) * t5331 * t3626 * t20795 * t21020 + F::cast_from(0.85748036236139473947e-3_f64) * t17747 * t3626 * t82293 * t12840 - F::cast_from(0.85748036236139473944e-3_f64) * t70914 - F::cast_from(0.68598428988911579154e-2_f64) * t59355 * t6611 - F::new(11.0) / F::new(162.0) * t70942 + t83699 / F::new(108.0) - t1222 * t5308 * t81202 / F::new(48.0) - t1222 * t5308 * t81198 / F::new(48.0) + t1222 * t5312 * t81177 / F::new(72.0) + t5373 * t21134 / F::new(18.0);
    t83712
}
