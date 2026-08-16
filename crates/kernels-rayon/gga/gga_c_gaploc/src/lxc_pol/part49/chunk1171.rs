//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1171/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1171(t3718: f64, t6553: f64, t12148: f64, t2355: f64, t1339: f64, t1537: f64, t46849: f64, t590: f64, t1441: f64, t493: f64, t41588: f64, t41592: f64, t41595: f64, t41600: f64, t41604: f64, t41607: f64, t41610: f64, t41613: f64, t41616: f64, t41619: f64) -> (f64, f64, f64) {
    let t47790 = t6553 * t3718;
    let t47791 = t2355 * t12148;
    let t47794 = t1537 * t1339 * t46849 * t590;
    let t47800 = t1441 * t493 * t46849 * t590;
    let t47802 = -0.25561950635947166451e1_f64 * t47794 + 0.9585731488480187419e0_f64 * t41588 - 0.57514388930881124514e0_f64 * t41592 - t41595 + t41600 - t41604 + 0.1022478025437886658e1_f64 * t47800 - t41607 - t41610 + t41613 + t41616 - t41619;
    (t47790, t47791, t47802)
}
