//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 802/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk802(t2679: f64, t3251: f64, t9800: f64, t9805: f64, t9893: f64, t28023: f64, t883: f64, t1967: f64, t5641: f64, t28640: f64, t28641: f64, t28668: f64) -> (f64, f64, f64, f64, f64) {
    let t40989 = t9800 * t3251 * t2679;
    let t41008 = t9805 * t9893 * t2679;
    let t41010 = t883 * t28023;
    let t41012 = t9800 * t1967 * t41010;
    let t41015 = t9805 * t5641 * t41010;
    let t41019 = t28640 * t28641 * t883 * t28668;
    (t40989, t41008, t41012, t41015, t41019)
}
