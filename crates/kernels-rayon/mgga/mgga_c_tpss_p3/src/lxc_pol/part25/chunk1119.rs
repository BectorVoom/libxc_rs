//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1119/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1119(t15248: f64, t15251: f64, t15292: f64, t15294: f64, t15296: f64, t15299: f64, t15301: f64, t15304: f64, t15307: f64, t15309: f64, t15312: f64, t11938: f64, t11958: f64, t15264: f64, t15268: f64, t15273: f64, t15277: f64, t15283: f64, t15288: f64, t15334: f64, t15339: f64, t15342: f64) -> (f64, f64) {
    let t15385 = -0.54771111111111111111e-1_f64 * t15248 + 0.29896666666666666667e0_f64 * t15251 + 0.1898925e1_f64 * t15292 + 0.3071625e0_f64 * t15294 + 0.18257037037037037037e-1_f64 * t15296 - 0.76790625e-1_f64 * t15299 + 0.3071625e0_f64 * t15301 + 0.15358125e0_f64 * t15304 + 0.142419375e1_f64 * t15307 - 0.1898925e1_f64 * t15309 - 0.9494625e0_f64 * t15312;
    let t15406 = -0.16431333333333333333e0_f64 * t15334 + 0.26574814814814814815e0_f64 * t11938 - t11958 - 0.19931111111111111111e0_f64 * t15283 + 0.36514074074074074075e-1_f64 * t15339 - 0.27385555555555555556e-1_f64 * t15342 - 0.39862222222222222222e0_f64 * t15268 - 0.11958666666666666667e1_f64 * t15264 + 0.11958666666666666667e1_f64 * t15277 + 0.17938e1_f64 * t15273 + 0.59793333333333333334e0_f64 * t15288;
    (t15385, t15406)
}
