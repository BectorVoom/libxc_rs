//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1116/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1116(t1038: f64, t15275: f64, t141: f64, t15271: f64, t15286: f64, t15266: f64, t2895: f64, t15262: f64, t15257: f64, t9185: f64, t15281: f64, t11938: f64, t12129: f64, t15264: f64, t15268: f64, t15273: f64, t15277: f64, t15283: f64, t15288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15320 = t1038 * t15275;
    let t15321 = t141 * t15320;
    let t15323 = t1038 * t15271;
    let t15324 = t141 * t15323;
    let t15326 = t1038 * t15286;
    let t15327 = t141 * t15326;
    let t15329 = t2895 * t15266;
    let t15330 = t141 * t15329;
    let t15333 = t2895 * t15262;
    let t15334 = t141 * t15333;
    let t15338 = t9185 * t15257;
    let t15339 = t141 * t15338;
    let t15341 = t2895 * t15281;
    let t15342 = t141 * t15341;
    let t15349 = -0.16557e0_f64 * t15334 + 0.26837777777777777779e0_f64 * t11938 - t12129 - 0.20128333333333333333e0_f64 * t15283 + 0.36793333333333333333e-1_f64 * t15339 - 0.27595e-1_f64 * t15342 - 0.40256666666666666666e0_f64 * t15268 - 0.12077e1_f64 * t15264 + 0.12077e1_f64 * t15277 + 0.181155e1_f64 * t15273 + 0.60385e0_f64 * t15288;
    (t15321, t15324, t15327, t15330, t15334, t15339, t15342, t15349)
}
