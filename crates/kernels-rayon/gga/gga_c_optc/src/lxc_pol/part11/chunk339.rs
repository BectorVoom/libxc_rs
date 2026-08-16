//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 339/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk339(t1476: f64, t415: f64, t1088: f64, t1091: f64, t1444: f64, t1451: f64, t1454: f64, t1457: f64) -> (f64, f64) {
    let t1477 = t1476 * t415;
    let t1483 = 0.258925e1_f64 * t1451 - t1088 - 0.301925e0_f64 * t1444 + 0.16504875e0_f64 * t1454 - t1091 - 0.82785e-1_f64 * t1457;
    (t1477, t1483)
}
