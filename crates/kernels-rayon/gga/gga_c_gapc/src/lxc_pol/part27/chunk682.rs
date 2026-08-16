//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 682/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk682(t5: f64, t825: f64, t102: f64, t2530: f64, t1033: f64, t291: f64, t332: f64, t327: f64, t966: f64, t818: f64, t2404: f64, t2553: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7089 = t825 * t5;
    let t7108 = t2530 * t102;
    let t7113 = t1033 * t291;
    let t7115 = t332 * t5;
    let t7120 = t1033 * t327;
    let t7122 = t966 * t5;
    let t7158 = t966 * t818;
    let t7165 = t2553 * t2404;
    (t7089, t7108, t7113, t7115, t7120, t7122, t7158, t7165)
}
