//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 733/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk733(t1364: f64, t3548: f64, t198: f64, t2115: f64, t2208: f64, t2217: f64, t2292: f64, t2302: f64, t2310: f64, t2333: f64, t2347: f64, t2351: f64, t2439: f64, t4706: f64, t4727: f64, t4743: f64, t4746: f64) -> f64 {
    let t4814 = t3548 * t1364;
    let t4817 = 6.0_f64 * t198 * t2115 * t4706 + 6.0_f64 * t2439 * t4814 - t2208 - t2217 - t2292 + t2302 + t2310 + t2333 + t2347 + t2351 + t4727 + t4743 + t4746;
    t4817
}
