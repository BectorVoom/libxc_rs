//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1112/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1112(t2036: f64, t2956: f64, t82: f64, t16129: f64, t237: f64, t6282: f64, t2318: f64, t1167: f64, t204: f64, t3981: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22085 = t2036 * t2956;
    let t22147 = 12.0_f64 * t82;
    let t22148 = 24.0_f64 * t16129;
    let t22180 = t237 * t6282;
    let t22185 = t237 * t2318;
    let t22230 = t204 * t3981 * t1167;
    (t22085, t22147, t22148, t22180, t22185, t22230)
}
