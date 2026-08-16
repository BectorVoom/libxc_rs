//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1062/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1062(t12288: f64, t7063: f64, t1125: f64, t3449: f64, t2469: f64, t11183: f64, t11186: f64, t12012: f64, t12013: f64, t12014: f64, t12015: f64, t12016: f64, t12017: f64, t12018: f64, t12019: f64, t12020: f64, t12021: f64, t12022: f64) -> (f64, f64, f64, f64) {
    let t12290 = 6.0_f64 * t7063 * t12288;
    let t12291 = t1125 * t3449;
    let t12293 = 2.0_f64 * t2469 * t12291;
    let t12580 = 0.5431140175846100239e-5_f64 * t11183 + 0.5431140175846100239e-5_f64 * t11186 - t12012 - t12013 - t12014 + t12015 + t12016 - t12017 + t12018 - t12019 - t12020 + t12021 + t12022;
    (t12290, t12291, t12293, t12580)
}
