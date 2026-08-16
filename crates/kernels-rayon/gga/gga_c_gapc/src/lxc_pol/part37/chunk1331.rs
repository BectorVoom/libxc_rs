//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1331/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1331(t11629: f64, t3254: f64, t10102: f64, t3724: f64, t10292: f64, t11684: f64, t11691: f64, t2165: f64, t24761: f64, t35682: f64, t6857: f64, t8452: f64) -> (f64, f64, f64, f64, f64) {
    let t35928 = t3254 * t11629;
    let t35930 = t10102 * t3724;
    let t35932 = t10292 * t11684;
    let t35934 = t2165 * t11691;
    let t35938 = t35682 * t24761 * t8452 * t6857;
    (t35928, t35930, t35932, t35934, t35938)
}
