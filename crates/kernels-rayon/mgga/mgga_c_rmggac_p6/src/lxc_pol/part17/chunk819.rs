//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 819/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk819(t2019: f64, t2339: f64, t7926: f64, t118: f64, t2001: f64, t2318: f64, t498: f64, t1986: f64, t495: f64, t36343: f64, t8457: f64, t1652: f64, t2084: f64, t2145: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t40201 = t2019 * t7926 * t2339;
    let t40231 = t2001 * t118 * t2318 * t498;
    let t40246 = t1986 * t118 * t2318 * t495;
    let t40250 = t36343 * t8457;
    let t40251 = 0.24829349937757072982e-4_f64 * t40250;
    let t40259 = t2145 * t27 * t2084 * t1652;
    (t40201, t40231, t40246, t40251, t40259)
}
