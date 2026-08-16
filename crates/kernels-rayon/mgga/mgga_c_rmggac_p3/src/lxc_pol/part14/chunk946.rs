//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 946/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk946(t36343: f64, t8457: f64, t1981: f64, t3142: f64, t508: f64, t8512: f64, t1652: f64, t2084: f64, t2145: f64, t27: f64, t16156: f64, t9213: f64) -> (f64, f64, f64, f64) {
    let t40250 = t36343 * t8457;
    let t40251 = 0.24829349937757072982e-4_f64 * t40250;
    let t40254 = t8512 * t1981 * t3142 * t508;
    let t40259 = t2145 * t27 * t2084 * t1652;
    let t40260 = 0.18183107769496894486e-1_f64 * t40259;
    let t40262 = t16156 * t9213;
    (t40251, t40254, t40260, t40262)
}
