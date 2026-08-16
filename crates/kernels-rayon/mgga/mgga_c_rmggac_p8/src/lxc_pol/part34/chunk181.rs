//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 181/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk181(t249: f64, t433: f64, t945: f64, t12: f64, t13: f64, t140: f64, t141: f64, t6: f64, t36: f64, t214: f64, t243: f64, t242: f64) -> (f64, f64, f64, f64, f64) {
    let t946 = t249 * t433;
    let t948 = 0.10843581300301739842e-1_f64 * t945 * t946;
    let t951 = 1.0_f64 / t13 / t12 * t140;
    let t952 = t141 * t6;
    let t953 = t952 * t36;
    let t954 = t951 * t953;
    let t956 = t243 * t214;
    let t957 = t242 * t956;
    (t948, t953, t954, t956, t957)
}
