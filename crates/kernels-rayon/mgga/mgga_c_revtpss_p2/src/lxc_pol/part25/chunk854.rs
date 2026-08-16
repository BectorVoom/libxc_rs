//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 854/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk854(t10073: f64, t4089: f64, t10008: f64, t10015: f64, t10020: f64, t10027: f64, t10032: f64, t10035: f64, t10041: f64, t10044: f64, t10049: f64, t10062: f64, t10066: f64, t10070: f64, t1437: f64, t213: f64, t3924: f64, t4004: f64, t4087: f64, t4118: f64, t546: f64, t5745: f64, t820: f64, t9840: f64, t9891: f64, t9899: f64) -> f64 {
    let t10074 = t10073 * t4089;
    let t10076 = -0.58544643236296698113e-1_f64 * t10015 - 0.29272321618148349057e-1_f64 * t10020 + 0.58544643236296698113e-1_f64 * t10027 - 0.65854491829355115987e0_f64 * t820 * t1437 * t9891 + 0.21951497276451705329e-1_f64 * t10032 + t10035 + 0.39512695097613069591e1_f64 * t5745 * t4087 * t9840 - 0.16463622957338778996e-1_f64 * t10041 - 0.19514881078765566038e-2_f64 * t10044 - 0.19756347548806534796e1_f64 * t820 * t4118 * t3924 + 0.39512695097613069591e1_f64 * t820 * t10049 * t4004 - 0.65854491829355115987e0_f64 * t820 * t1437 * t9899 + 0.65854491829355115987e0_f64 * t213 * t546 * t10008 - 0.32927245914677557992e-1_f64 * t10062 + 0.16463622957338778996e-1_f64 * t10066 - 0.21951497276451705329e-1_f64 * t10070 + 0.19514881078765566038e-2_f64 * t10074;
    t10076
}
