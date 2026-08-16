//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 407/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk407(t1165: f64, t1167: f64, t1169: f64, t1154: f64, t1161: f64, t14: f64, t2063: f64, t2067: f64, t351: f64, t740: f64, t705: f64, t78: f64) -> (f64, f64) {
    let t2075 = -0.99474444444444444447e-4_f64 * t1165 + 0.19894888888888888889e-3_f64 * t1167 + 0.52442777777777777777e-2_f64 * t1169;
    let t2078 = -t2063 * t1154 / 18.0_f64 - t2067 * t351 / 6.0_f64 + t740 * t1161 / 9.0_f64 + t14 * t2075 / 2.0_f64;
    let t2084 = t78 * t705;
    (t2078, t2084)
}
