//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 500/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk500(t3399: f64, t40: f64, t1264: f64, t138: f64, t1: f64, t2060: f64, t123: f64, t1256: f64, t1278: f64, t654: f64, t130: f64, t635: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3400 = t40 * t3399;
    let t3406 = t1264 * t138;
    let t3411 = t2060 * t1;
    let t3412 = t123 * t1256;
    let t3437 = t654 * t1278;
    let t3439 = t130 * t635;
    (t3400, t3406, t3411, t3412, t3437, t3439)
}
