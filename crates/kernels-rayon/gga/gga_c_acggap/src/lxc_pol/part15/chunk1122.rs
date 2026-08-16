//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1122/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1122(t31346: f64, t5932: f64, t7433: f64, t9637: f64, t1773: f64, t2030: f64, t2031: f64, t1181: f64, t5537: f64, t7351: f64, t7564: f64, t5796: f64, t7822: f64) -> (f64, f64, f64, f64, f64) {
    let t39391 = t31346 * t5932;
    let t39393 = t7433 * t9637;
    let t39402 = t2030 * t1773 * t2031;
    let t39406 = t7564 * t1181 * t7351 * t5537;
    let t39412 = t7822 * t5796;
    (t39391, t39393, t39402, t39406, t39412)
}
