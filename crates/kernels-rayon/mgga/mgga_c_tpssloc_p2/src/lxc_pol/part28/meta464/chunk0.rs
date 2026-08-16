//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1673/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1673(t25097: f64, t815: f64, t23097: f64, t1894: f64, t236: f64, t4119: f64, t6591: f64, t23062: f64, t7497: f64, t1510: f64, t776: f64, t13223: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25098 = t815 * t25097;
    let t25099 = t23097 * t25098;
    let t25106 = t1894 * t236 * t4119;
    let t25107 = t6591 * t25106;
    let t25109 = t23062 * t7497;
    let t25111 = t1510 * t776;
    let t25112 = t815 * t25111;
    let t25113 = t23097 * t25112;
    let t25115 = t13223 * t232;
    (t25098, t25099, t25106, t25107, t25109, t25111, t25112, t25113, t25115)
}
