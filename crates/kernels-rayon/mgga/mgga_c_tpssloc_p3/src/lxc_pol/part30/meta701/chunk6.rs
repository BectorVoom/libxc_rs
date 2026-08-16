//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2271/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2271(t23384: f64, t28481: f64, t14529: f64, t1539: f64, t17582: f64, t18061: f64, t1927: f64, t1956: f64, t23327: f64, t23346: f64, t23394: f64, t25784: f64, t28705: f64, t4548: f64, t4664: f64, t60971: f64, t61058: f64, t6687: f64, t6704: f64, t7625: f64, t82402: f64, t88100: f64, t88102: f64, t88152: f64, t88772: f64) -> f64 {
    let t99151 = t23384 * t28481;
    let t99172 = t88100 + t88102 + 0.14621636149762012769e-1_f64 * t82402 * t28705 + 0.21932454224643019153e-1_f64 * t23346 * t28481 - 2.0_f64 * t61058 * t1956 - 0.27415567780803773942e-2_f64 * t99151 + 0.16449340668482264365e-1_f64 * t1927 * t4548 * t25784 + 0.10966227112321509577e-1_f64 * t23327 * t88772 * t1539 * t4664 - 2.0_f64 * t14529 * t7625 - 2.0_f64 * t60971 * t1956 + 0.16449340668482264365e-1_f64 * t6687 * t6704 * t23394 * t18061 + 0.3289868133696452873e-1_f64 * t6687 * t6704 * t23394 * t17582 - t88152;
    t99172
}
