//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2798/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2798<F: Float>(t1573: F, t40317: F, t39692: F, t39694: F, t39697: F, t39701: F, t39707: F, t4514: F, t51380: F, t51435: F, t51438: F, t51442: F, t51445: F, t837: F) -> F {
    let t51452 = t40317 * t1573;
    let t51456 = t51435 + F::cast_from(0.32927245914677557992e-1_f64) * t51438 - F::cast_from(0.29272321618148349057e-1_f64) * t51442 + F::cast_from(0.30356481678079769392e-1_f64) * t51445 - F::cast_from(0.29272321618148349057e-1_f64) * t39692 + F::cast_from(0.19514881078765566037e-2_f64) * t39694 + t39697 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t51380 * t837 + F::cast_from(0.11044544084478153697e-3_f64) * t51452 - F::cast_from(0.58911598146606471822e-3_f64) * t39701 + F::cast_from(0.16463622957338778996e-1_f64) * t39707;
    t51456
}
