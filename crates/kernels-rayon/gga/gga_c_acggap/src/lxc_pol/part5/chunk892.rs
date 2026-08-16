//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 892/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk892(t1133: f64, t3573: f64, t1: f64, t1161: f64, t1171: f64, t1170: f64) -> (f64, f64, f64) {
    let t13282 = t3573 * t1133;
    let t13285 = t1161 * t1171 * t1;
    let t13286 = t1170 * t13285;
    (t13282, t13285, t13286)
}
