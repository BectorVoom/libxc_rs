//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1120/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1120(t1711: f64, t715: f64, t11893: f64, t15050: f64, t18217: f64, t6024: f64, t807: f64, t11910: f64, t18222: f64, t11916: f64, t11922: f64, t11898: f64, t11900: f64, t11906: f64, t11909: f64, t11914: f64, t11921: f64, t11938: f64, t11944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20030 = t715 * t1711;
    let t20031 = 32.0_f64 * t20030;
    let t20032 = 0.70178683471615754484e1_f64 * t11893;
    let t20033 = 2.0_f64 * t15050;
    let t20034 = 8.0_f64 * t18217;
    let t20035 = t6024 * t807;
    let t20036 = 0.24415263074675393405e-3_f64 * t20035;
    let t20037 = 120.0_f64 * t11910;
    let t20038 = 120.0_f64 * t18222;
    let t20039 = 480.0_f64 * t11916;
    let t20040 = 0.11696447245269292414e1_f64 * t11922;
    let t20041 = -t20031 + t20032 + t11898 + t20033 + t11900 + t11906 + t20034 - t11909 + t20036 + t20037 - t11914 + t20038 - t20039 - t11921 - t20040 - t11938 - t11944;
    (t20031, t20032, t20033, t20034, t20036, t20037, t20038, t20039, t20040, t20041)
}
