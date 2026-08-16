//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2162/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2162(t1831: f64, t91191: f64, t26257: f64, t5314: f64, t28100: f64, t80853: f64, t80855: f64, t80767: f64, t80776: f64, t80780: f64, t91206: f64, t91215: f64, t91226: f64, t97310: f64, t97315: f64, t97318: f64, t97320: f64, t97322: f64, t97326: f64, t97333: f64, t97337: f64, t97340: f64) -> f64 {
    let t97342 = t91191 * t1831;
    let t97344 = t26257 * t5314;
    let t97347 = t80853 * t80855 * t28100;
    let t97349 = -t97310 / 96.0_f64 + 0.33643963411783659045e-4_f64 * t97315 + t97318 / 1536.0_f64 + t97320 / 384.0_f64 + t97322 / 192.0_f64 - 0.20186378047070195427e-3_f64 * t97326 - 0.63250651214153279005e-2_f64 * t91206 - t91215 - 0.67826230238155856634e-1_f64 * t80767 - 35.0_f64 / 216.0_f64 * t80776 + t91226 - 0.12111826828242117256e-2_f64 * t97333 + 0.40372756094140390854e-3_f64 * t97337 - 0.31625325607076639503e-2_f64 * t80780 - t97340 / 384.0_f64 - t97342 / 192.0_f64 - t97344 / 192.0_f64 - 0.40372756094140390853e-3_f64 * t97347;
    t97349
}
