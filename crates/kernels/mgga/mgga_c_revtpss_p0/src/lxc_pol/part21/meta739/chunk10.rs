//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2602/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2602<F: Float>(t14109: F, t47603: F, t9681: F, t14268: F, t3915: F, t686: F, t72: F, t14293: F, t9664: F, t13739: F, t1445: F, t4071: F, t47525: F, t47527: F, t47531: F, t47534: F, t47537: F, t47540: F, t47893: F, t47899: F, t47904: F, t47907: F, t47909: F) -> F {
    let t47913 = t47603 * t14109 * t9681;
    let t47918 = t3915 * t14268 * t72 * t686;
    let t47920 = t14293 * t9664;
    let t47922 = F::cast_from(0.39029762157531132075e-1_f64) * t47525 - F::cast_from(0.58544643236296698113e-1_f64) * t47893 + F::cast_from(0.39512695097613069591e1_f64) * t4071 * t13739 + F::cast_from(0.78059524315062264151e-1_f64) * t47527 + F::cast_from(0.58544643236296698114e-1_f64) * t47531 - F::cast_from(0.39029762157531132075e-2_f64) * t47899 + F::cast_from(0.19514881078765566037e-2_f64) * t47534 + F::cast_from(0.32927245914677557992e-1_f64) * t47537 - F::cast_from(0.30356481678079769392e-1_f64) * t47904 + F::cast_from(0.19514881078765566037e-2_f64) * t47907 - F::cast_from(0.19756347548806534796e1_f64) * t47909 * t1445 - F::cast_from(0.17563392970889009433e0_f64) * t47913 + F::cast_from(0.16463622957338778996e-1_f64) * t47540 - F::cast_from(0.29272321618148349057e-1_f64) * t47918 - F::cast_from(0.46263278077393568556e-2_f64) * t47920;
    t47922
}
