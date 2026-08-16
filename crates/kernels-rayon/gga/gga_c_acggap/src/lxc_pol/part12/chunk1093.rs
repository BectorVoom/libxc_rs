//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1093/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1093(t17972: f64, t2068: f64, t2263: f64, t30984: f64, t8649: f64, t30934: f64, t8602: f64, t31346: f64, t4732: f64, t1165: f64, t4533: f64, t7351: f64, t7575: f64) -> (f64, f64, f64, f64, f64) {
    let t35454 = t2068 * t17972 * t2263;
    let t35456 = t30984 * t8649;
    let t35458 = t30934 * t8602;
    let t35460 = t31346 * t4732;
    let t35464 = t7575 * t1165 * t7351 * t4533;
    (t35454, t35456, t35458, t35460, t35464)
}
