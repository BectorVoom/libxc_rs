//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1035/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1035(t30248: f64, t532: f64, t537: f64, t7637: f64, t8859: f64, t1576: f64, t7614: f64, t1181: f64, t5249: f64, t599: f64, t7493: f64, t4718: f64, t604: f64, t7426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36231 = t30248 * t532;
    let t36236 = t30248 * t537;
    let t36238 = t7637 * t8859;
    let t36240 = t7614 * t1576;
    let t36273 = t7493 * t1181 * t599 * t5249;
    let t36283 = t7426 * t1181 * t604 * t4718;
    (t36231, t36236, t36238, t36240, t36273, t36283)
}
