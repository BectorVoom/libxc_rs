//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 787/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk787(t10615: f64, t1423: f64, t3129: f64, t40377: f64, t2890: f64, t9267: f64, t9278: f64, t20671: f64, t31047: f64, t34814: f64, t26984: f64, t9294: f64) -> (f64, f64, f64, f64, f64) {
    let t42156 = t10615 * t1423 * t3129;
    let t42170 = 0.19171462976960374838e0_f64 * t40377;
    let t42183 = t9267 * t2890 * t9278;
    let t42187 = t31047 * t20671 * t34814;
    let t42189 = t26984 * t9294;
    (t42156, t42170, t42183, t42187, t42189)
}
