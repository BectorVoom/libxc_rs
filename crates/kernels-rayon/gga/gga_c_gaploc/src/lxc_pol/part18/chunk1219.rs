//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1219/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1219(t21446: f64, t2508: f64, t3009: f64, t7226: f64, t21783: f64, t1850: f64, t29273: f64, t29280: f64, t32253: f64, t32256: f64, t32259: f64, t32261: f64, t32266: f64, t32269: f64, t32272: f64, t32275: f64, t32277: f64, t32281: f64, t5396: f64) -> f64 {
    let t32285 = 0.92286314761706691402e-1_f64 * t2508 * t7226 * t3009 * t21446;
    let t32289 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t3009 * t21783;
    let t32290 = -t32253 - t32256 + t29273 - t29280 - t32259 - 0.17090058289204942853e-2_f64 * t1850 * t5396 * t32261 + t32266 + t32269 - t32272 + t32275 - t32277 - t32281 - t32285 - t32289;
    t32290
}
