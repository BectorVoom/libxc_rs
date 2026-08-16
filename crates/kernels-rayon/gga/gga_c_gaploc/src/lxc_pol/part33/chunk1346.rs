//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1346/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1346(t35043: f64, t1339: f64, t31585: f64, t1537: f64, t590: f64, t31590: f64, t10474: f64, t4428: f64, t30830: f64, t7967: f64, t913: f64, t10609: f64, t31054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35044 = 0.59584149919750711116e-1_f64 * t35043;
    let t35045 = t1339 * t31585;
    let t35048 = 0.51123901271894332902e1_f64 * t1537 * t35045 * t590;
    let t35052 = 0.51123901271894332902e1_f64 * t1537 * t1339 * t31590 * t590;
    let t35054 = 0.2044956050875773316e1_f64 * t4428 * t10474;
    let t35074 = t30830 * t913 * t7967;
    let t35075 = 0.59584149919750711116e-1_f64 * t35074;
    let t35089 = t31054 * t10609;
    (t35044, t35048, t35052, t35054, t35075, t35089)
}
