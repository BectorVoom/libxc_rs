//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 487/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk487(t2389: f64, t282: f64, t129: f64, t918: f64, t923: f64, t617: f64, t2404: f64, t332: f64, t298: f64, t181: f64, t2394: f64, t2254: f64, t314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2706 = t2389 * t282;
    let t2707 = t2706 * t129;
    let t2712 = t918 * t923;
    let t2713 = t617 * t2712;
    let t2716 = t332 * t2404;
    let t2717 = t298 * t2716;
    let t2718 = t181 * t2717;
    let t2721 = t2394 * t282;
    let t2722 = t2721 * t129;
    let t2723 = t314 * t2254;
    (t2706, t2707, t2712, t2713, t2716, t2718, t2721, t2722, t2723)
}
