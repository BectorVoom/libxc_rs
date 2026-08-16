//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1177/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1177(t1888: f64, t232: f64, t6646: f64, t68217: f64, t67392: f64, t67350: f64, t82018: f64, t9975: f64, t22996: f64, t2632: f64, t67405: f64, t25038: f64, t25248: f64, t25249: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105601 = t1888 * t6646 * t68217 * t232;
    let t105621 = t1888 * t6646 * t67392 * t232;
    let t105629 = t1888 * t82018 * t67350 * t9975;
    let t105634 = t1888 * t22996 * t67350 * t2632;
    let t105638 = t1888 * t6646 * t67350 * t232;
    let t105642 = t1888 * t6646 * t67405 * t232;
    let t105646 = t25038 * t25248 * t25249 * t5544;
    (t105601, t105621, t105629, t105634, t105638, t105642, t105646)
}
