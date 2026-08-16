//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 638/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk638(t1165: f64, t1552: f64, t6151: f64, t1539: f64, t5852: f64, t1163: f64, t1175: f64, t5862: f64, t1140: f64, t1784: f64, t336: f64, t337: f64, t5506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6153 = t1165 * t1552 * t6151;
    let t6157 = t1165 * t5852 * t1539;
    let t6158 = t1163 * t6157;
    let t6161 = t1165 * t5862 * t1175;
    let t6164 = t1140 * t1784;
    let t6167 = t336 * t337 * t5506;
    (t6153, t6157, t6158, t6161, t6164, t6167)
}
