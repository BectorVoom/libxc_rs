//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1060/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1060(t11237: f64, t11252: f64, t11263: f64, t12025: f64, t12026: f64, t12027: f64, t12028: f64, t12030: f64, t12031: f64, t12033: f64, t12034: f64, t12035: f64, t12036: f64) -> f64 {
    let t12584 = -0.5431140175846100239e-5_f64 * t11237 - t12025 + t12026 - t12027 + t12028 - 0.59742541934307102629e-4_f64 * t11252 + t12030 + t12031 - 0.5431140175846100239e-5_f64 * t11263 + t12033 - t12034 - t12035 + t12036;
    t12584
}
