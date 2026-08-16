//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1202/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1202(t1173: f64, t1181: f64, t13308: f64, t13310: f64, t13314: f64, t13317: f64, t13320: f64, t13330: f64, t13332: f64, t13337: f64, t1532: f64, t16814: f64, t16818: f64, t360: f64, t4289: f64, t5710: f64, t6258: f64) -> f64 {
    let t21894 = 0.42874018118069736972e-3_f64 * t13308 + 0.11337795902333997111e-1_f64 * t13310 - t13314 + t13317 - t13320 + t13330 - 0.40015750243531754508e-2_f64 * t13332 + t13337 + 0.17149607247227894789e-2_f64 * t16814 + 0.85748036236139473944e-3_f64 * t16818 + 0.68598428988911579156e-2_f64 * t1173 * t1181 * t4289 * t5710 + 0.68598428988911579156e-2_f64 * t1173 * t1181 * t1532 * t6258 * t360;
    t21894
}
