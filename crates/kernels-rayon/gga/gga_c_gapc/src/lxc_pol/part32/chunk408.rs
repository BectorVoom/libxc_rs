//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 408/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk408(t737: f64, t88: f64, t256: f64, t62: f64, t748: f64, t1150: f64, t19: f64, t252: f64, t1: f64, t348: f64, t745: f64, t1165: f64, t1167: f64, t1169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2053 = t88 * t737;
    let t2056 = t256 * t256;
    let t2057 = 1.0_f64 / t2056;
    let t2058 = t62 * t2057;
    let t2059 = t748 * t748;
    let t2063 = t1150 * t252 * t19;
    let t2067 = t348 * t745 * t1;
    let t2075 = -0.99474444444444444447e-4_f64 * t1165 + 0.19894888888888888889e-3_f64 * t1167 + 0.52442777777777777777e-2_f64 * t1169;
    (t2053, t2058, t2059, t2063, t2067, t2075)
}
