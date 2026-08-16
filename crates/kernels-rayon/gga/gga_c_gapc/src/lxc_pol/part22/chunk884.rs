//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 884/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk884(t10072: f64, t3411: f64, t1084: f64, t8711: f64, t134: f64, t7877: f64, t442: f64, t7591: f64, t941: f64, t10024: f64, t10027: f64, t10032: f64, t10037: f64, t10041: f64, t10044: f64, t10048: f64, t10051: f64, t10054: f64, t10059: f64, t10064: f64, t10070: f64) -> (f64, f64, f64, f64, f64) {
    let t10073 = t3411 * t10072;
    let t10075 = t1084 * t8711;
    let t10077 = t134 * t7877;
    let t10078 = t10077 * t442;
    let t10079 = t7591 * t941 * t10078;
    let t10080 = t10075 * t10079;
    let t10082 = -0.2204045389310251527e-6_f64 * t10024 + 0.6487109086417285278e-2_f64 * t10027 + 0.50027140879067581468e-8_f64 * t10032 - 0.16882049790461501058e-6_f64 * t10037 - 0.56273499301538336859e-8_f64 * t10041 + 0.4103275990737170396e-9_f64 * t10044 - 0.2813674965076916843e-7_f64 * t10048 + 0.10120442708333333334e-4_f64 * t10051 - 0.24619655944423022376e-7_f64 * t10054 - 0.24619655944423022376e-7_f64 * t10059 + 0.84410248952307505288e-7_f64 * t10064 - 0.25323074685692251586e-6_f64 * t10070 + 0.16882049790461501058e-6_f64 * t10073 - 0.17951832459475120482e-8_f64 * t10080;
    (t10073, t10078, t10079, t10080, t10082)
}
