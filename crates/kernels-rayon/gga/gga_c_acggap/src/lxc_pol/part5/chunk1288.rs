//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1288/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1288(t6121: f64, t997: f64, t1886: f64, t3228: f64, t1891: f64, t1008: f64, t5690: f64, t1165: f64, t1180: f64, t1181: f64, t18475: f64, t18480: f64, t18482: f64, t18485: f64, t18487: f64, t18489: f64, t22710: f64, t3176: f64, t3462: f64, t530: f64, t5922: f64) -> f64 {
    let t23891 = t997 * t6121;
    let t23893 = t3228 * t1886;
    let t23895 = t3228 * t1891;
    let t23897 = t1008 * t5690;
    let t23909 = 0.85748036236139473944e-3_f64 * t1180 * t1165 * t5922 * t3176 - 0.40015750243531754508e-2_f64 * t23891 + 0.42874018118069736972e-2_f64 * t23893 - 0.85748036236139473944e-3_f64 * t23895 - 0.17149607247227894789e-2_f64 * t23897 - 0.68598428988911579156e-2_f64 * t18475 - 0.34299214494455789578e-2_f64 * t18480 + 0.13719685797782315831e-1_f64 * t18482 + 0.68598428988911579156e-2_f64 * t18485 + 7.0_f64 / 72.0_f64 * t18487 + 7.0_f64 / 144.0_f64 * t18489 - 0.68598428988911579156e-2_f64 * t3462 * t1181 * t530 * t22710;
    t23909
}
