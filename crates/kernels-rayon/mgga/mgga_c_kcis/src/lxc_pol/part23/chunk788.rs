//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 788/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk788(t1420: f64, t4016: f64, t4031: f64, t532: f64, t1401: f64, t4039: f64, t4142: f64, t4178: f64, t25: f64, t4008: f64, t493: f64, t499: f64, t737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12087 = t4016 * t1420;
    let t12089 = t532 * t4031;
    let t12091 = t1401 * t4039;
    let t12119 = t4142 * t4178;
    let t12124 = t25 * t4008;
    let t12125 = t493 * t12124;
    let t12127 = t737 * t499;
    (t12087, t12089, t12091, t12119, t12125, t12127)
}
