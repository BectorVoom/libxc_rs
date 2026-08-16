//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1064/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1064(t2672: f64, t5679: f64, t6115: f64, t935: f64, t10913: f64, t2021: f64, t1980: f64, t7512: f64, t4370: f64, t6109: f64, t787: f64, t1: f64, t21888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22166 = t5679 * t2672;
    let t22238 = t6115 * t935;
    let t22242 = t2021 * t10913;
    let t22263 = t1980 * t7512;
    let t22274 = t787 * t6109 * t4370;
    let t22295 = t787 * t21888 * t1;
    (t22166, t22238, t22242, t22263, t22274, t22295)
}
