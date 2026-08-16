//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 850/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk850(t1063: f64, t2229: f64, t2453: f64, t3259: f64, t2436: f64, t3234: f64, t3240: f64, t2452: f64, t3239: f64, t6773: f64, t3258: f64, t2437: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10187 = t2229 * t1063;
    let t10189 = t2453 * t3259;
    let t10191 = t3234 * t2436;
    let t10192 = t10191 * t3240;
    let t10194 = t3234 * t2452;
    let t10195 = t10194 * t3259;
    let t10197 = t3239 * t6773;
    let t10198 = t3258 * t10197;
    let t10201 = t2437 * t3240;
    (t10187, t10189, t10192, t10195, t10198, t10201)
}
