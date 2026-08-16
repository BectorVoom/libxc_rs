//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1197/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1197(t7839: f64, t9674: f64, t7564: f64, t8480: f64, t8613: f64, t1181: f64, t604: f64, t6079: f64, t7426: f64, t6218: f64, t7575: f64, t6198: f64, t7351: f64) -> (f64, f64, f64, f64, f64) {
    let t40490 = t7839 * t9674;
    let t40493 = t7564 * t8480 * t8613;
    let t40497 = t7426 * t1181 * t604 * t6079;
    let t40501 = t7575 * t1181 * t604 * t6218;
    let t40505 = t7564 * t1181 * t7351 * t6198;
    (t40490, t40493, t40497, t40501, t40505)
}
