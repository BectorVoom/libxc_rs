//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 877/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk877(t1063: f64, t2229: f64, t2453: f64, t3259: f64, t2436: f64, t3234: f64, t3240: f64, t2452: f64, t3239: f64, t6773: f64, t3258: f64, t10172: f64, t10174: f64, t10176: f64, t10178: f64, t10183: f64, t10185: f64) -> f64 {
    let t10187 = t2229 * t1063;
    let t10189 = t2453 * t3259;
    let t10191 = t3234 * t2436;
    let t10192 = t10191 * t3240;
    let t10194 = t3234 * t2452;
    let t10195 = t10194 * t3259;
    let t10197 = t3239 * t6773;
    let t10198 = t3258 * t10197;
    let t10200 = 0.56366309740899397906e-3_f64 * t10172 - 0.93943849568165663176e-3_f64 * t10174 - 0.93943849568165663176e-3_f64 * t10176 + 0.28183154870449698953e-3_f64 * t10178 + 0.82073827867876094584e-5_f64 * t10183 - 0.2087641101514792515e-3_f64 * t10185 + 0.74372214241464483348e-4_f64 * t10187 - 0.74372214241464483348e-4_f64 * t10189 + 0.11742981196020707897e-4_f64 * t10192 + 0.11742981196020707897e-4_f64 * t10195 + 0.58714905980103539485e-5_f64 * t10198;
    t10200
}
