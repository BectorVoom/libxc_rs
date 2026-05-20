//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2728/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2728<F: Float>(t50186: F, t10495: F, t14978: F, t14979: F, t1580: F, t2765: F, t2770: F, t39549: F, t39550: F, t39554: F, t41008: F, t4474: F, t50155: F, t50164: F, t50166: F, t50169: F, t50174: F, t50178: F, t50184: F, t865: F, t886: F) -> F {
    let t50187 = F::cast_from(0.39029762157531132076e-1_f64) * t50186;
    let t50190 = -F::cast_from(0.11044544084478153697e-3_f64) * t50155 + F::cast_from(0.39512695097613069591e1_f64) * t865 * t2770 * t14978 * t886 + F::cast_from(0.98781737744032673976e-1_f64) * t50164 - F::cast_from(0.17073386770573548589e-1_f64) * t50166 - F::cast_from(0.32927245914677557992e-1_f64) * t50169 - F::cast_from(0.65854491829355115987e0_f64) * t41008 * t1580 + F::cast_from(0.16463622957338778996e-1_f64) * t50174 - F::cast_from(0.19637199382202157274e-3_f64) * t50178 - t39549 - F::cast_from(0.33133632253434461091e-3_f64) * t39550 - F::cast_from(0.19756347548806534796e1_f64) * t2765 * t14979 - t50184 + t50187 + t39554 + F::cast_from(0.39512695097613069591e1_f64) * t4474 * t10495;
    t50190
}
