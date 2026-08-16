//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 513/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk513(t1347: f64, t809: f64, t1359: f64, t828: f64, t1366: f64, t2476: f64, t241: f64) -> (f64, f64, f64, f64) {
    let t3716 = t1347 * t809;
    let t3754 = t1359 * t828;
    let t3780 = t1366 * t2476;
    let t3788 = t241 * t1359;
    (t3716, t3754, t3780, t3788)
}
