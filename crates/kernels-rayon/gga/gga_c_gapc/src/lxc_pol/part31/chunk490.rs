//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 490/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk490(t641: f64, t928: f64, t655: f64, t2299: f64, t332: f64, t330: f64, t197: f64, t617: f64, t968: f64, t2188: f64, t918: f64, t1904: f64, t2660: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2741 = t928 * t641;
    let t2744 = t928 * t655;
    let t2747 = t332 * t2299;
    let t2748 = t330 * t2747;
    let t2749 = t197 * t2748;
    let t2752 = t617 * t968;
    let t2755 = t332 * t2188;
    let t2756 = t918 * t2755;
    let t2757 = t197 * t2756;
    let t2760 = t2660 * t1904;
    (t2741, t2744, t2749, t2752, t2757, t2760)
}
