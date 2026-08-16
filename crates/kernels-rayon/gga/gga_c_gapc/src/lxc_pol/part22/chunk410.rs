//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 410/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk410(t132: f64, t762: f64, t737: f64, t88: f64, t256: f64, t62: f64, t748: f64, t1150: f64, t19: f64, t252: f64, t1: f64, t348: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2046 = t132 * t762;
    let t2053 = t88 * t737;
    let t2056 = t256 * t256;
    let t2057 = 1.0_f64 / t2056;
    let t2058 = t62 * t2057;
    let t2059 = t748 * t748;
    let t2063 = t1150 * t252 * t19;
    let t2067 = t348 * t745 * t1;
    (t2046, t2053, t2058, t2059, t2063, t2067)
}
