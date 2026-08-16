//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 608/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk608(t2672: f64, t4941: f64, t1: f64, t313: f64, t297: f64, t312: f64, t4961: f64, t894: f64, t123: f64, t323: f64, t287: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4996 = t4941 * t2672;
    let t4997 = t4996 * t1;
    let t4998 = t313 * t4997;
    let t5002 = t4941 * t1 * t297;
    let t5003 = t313 * t5002;
    let t5006 = t312 * t4961;
    let t5007 = t5006 * t297;
    let t5008 = t894 * t5007;
    let t5011 = t4996 * t123;
    let t5012 = t323 * t5011;
    let t5016 = t4941 * t123 * t297;
    let t5017 = t323 * t5016;
    let t5021 = t287 * t4961 * t297;
    let t5022 = t914 * t5021;
    (t4997, t4998, t5002, t5003, t5007, t5008, t5011, t5012, t5016, t5017, t5021, t5022)
}
