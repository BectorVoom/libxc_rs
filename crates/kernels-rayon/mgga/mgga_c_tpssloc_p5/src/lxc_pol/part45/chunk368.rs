//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 368/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk368(t2250: f64, t31: f64, t65: f64, t608: f64, t628: f64, t36: f64, t365: f64, t42: f64, t2244: f64, t43: f64, t54: f64, t55: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2251 = t31 * t2250;
    let t2252 = t2251 * t65;
    let t2255 = t608 * t628;
    let t2261 = 1.0_f64 / t36 / t365;
    let t2262 = sigma0 * t2261;
    let t2267 = 1.0_f64 / t42;
    let t2268 = t2267 * t2244;
    let t2271 = t43 * t2250;
    let t2274 = 1.0_f64 / t54;
    let t2275 = t2274 * t2244;
    let t2278 = t55 * t2250;
    (t2251, t2252, t2255, t2261, t2262, t2267, t2268, t2271, t2275, t2278)
}
