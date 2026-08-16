//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 361/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk361(t201: f64, t109: f64, t198: f64, t212: f64, t410: f64, t1297: f64, t1301: f64, t1304: f64, t193: f64, t202: f64, t210: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1305 = t201 * t201;
    let t1306 = 1.0_f64 / t1305;
    let t1310 = t109 * tau1;
    let t1311 = t198 * t212;
    let t1314 = t410 * tau1;
    let t1315 = t1314 * t198;
    let t1318 = -0.10241644597362152e-1_f64 * t193 * t1297 * t202 + 0.39334231522004008709e-4_f64 * t1301 * t1304 * t1306 + 5.0_f64 / 3.0_f64 * t1310 * t1311 + 5.0_f64 / 3.0_f64 * t210 * t1315;
    (t1305, t1306, t1310, t1311, t1314, t1315, t1318)
}
