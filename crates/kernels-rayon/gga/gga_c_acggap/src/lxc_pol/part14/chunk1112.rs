//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1112/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1112(t1773: f64, t2030: f64, t2031: f64, t1181: f64, t5537: f64, t7351: f64, t7564: f64, t5796: f64, t7822: f64, t5801: f64, t6226: f64, t1165: f64, t6198: f64, t8600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39402 = t2030 * t1773 * t2031;
    let t39406 = t7564 * t1181 * t7351 * t5537;
    let t39412 = t7822 * t5796;
    let t39414 = t7822 * t5801;
    let t39418 = t7564 * t1181 * t7351 * t6226;
    let t39422 = t7564 * t1165 * t8600 * t6198;
    (t39402, t39406, t39412, t39414, t39418, t39422)
}
