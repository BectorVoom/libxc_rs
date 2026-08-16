//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 677/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk677(t2312: f64, t2322: f64, t4144: f64, t883: f64, t485: f64, t2316: f64, t1624: f64, t2321: f64, t882: f64, t2327: f64, t484: f64, t119: f64, t3831: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6293 = t2312 * t2322;
    let t6295 = t883 * t4144;
    let t6296 = t485 * t6295;
    let t6297 = t2316 * t6296;
    let t6299 = t1624 * t2321;
    let t6300 = t882 * t6299;
    let t6302 = t484 * t2327;
    let t6305 = t481 * t3831 * t119;
    (t6293, t6295, t6297, t6300, t6302, t6305)
}
