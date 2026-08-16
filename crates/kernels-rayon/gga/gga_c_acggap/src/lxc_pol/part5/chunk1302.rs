//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1302/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1302(t1165: f64, t1173: f64, t1180: f64, t1181: f64, t1188: f64, t18743: f64, t18747: f64, t18763: f64, t18765: f64, t21342: f64, t24173: f64, t24175: f64, t24184: f64, t24194: f64, t24196: f64, t24201: f64, t301: f64, t335: f64, t336: f64, t4437: f64, t4680: f64, t530: f64, t5867: f64, t6395: f64) -> f64 {
    let t24204 = -0.10289764348336736873e-1_f64 * t18743 - 0.68598428988911579156e-2_f64 * t18747 - 0.17149607247227894789e-2_f64 * t24173 - 0.32012600194825403606e-1_f64 * t24175 + 0.34299214494455789578e-2_f64 * t1173 * t1181 * t530 * t21342 + 0.34299214494455789578e-2_f64 * t1180 * t4680 * t6395 + 0.85748036236139473944e-3_f64 * t1180 * t1165 * t24184 * t1188 + 0.42874018118069736972e-3_f64 * t1180 * t1165 * t5867 * t4437 - 0.34299214494455789578e-2_f64 * t18763 - 0.40015750243531754508e-2_f64 * t24194 - t335 * t336 * t24196 * t301 / 24.0_f64 - 0.17149607247227894789e-2_f64 * t24201 - 0.42874018118069736972e-3_f64 * t18765;
    t24204
}
