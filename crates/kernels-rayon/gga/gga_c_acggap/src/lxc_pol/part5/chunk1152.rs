//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1152/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1152(t2937: f64, t495: f64, t5746: f64, t943: f64, t1026: f64, t1027: f64, t1165: f64, t1180: f64, t1181: f64, t12814: f64, t13591: f64, t1532: f64, t1574: f64, t15746: f64, t1894: f64, t1899: f64, t20737: f64, t20739: f64, t20753: f64, t20764: f64, t2325: f64, t3169: f64, t3176: f64, t3462: f64, t386: f64, t418: f64, t4847: f64, t4876: f64, t5679: f64, t6119: f64) -> f64 {
    let t20769 = t2937 * t495;
    let t20775 = t5746 * t943;
    let t20781 = 0.85748036236139473944e-3_f64 * t20737 + 0.85748036236139473944e-3_f64 * t20739 + 0.42874018118069736972e-3_f64 * t418 * t386 * t4847 * t1894 + 0.85748036236139473944e-3_f64 * t418 * t386 * t1574 * t6119 + 0.85748036236139473945e-2_f64 * t418 * t1026 * t5679 * t1027 + 0.17149607247227894789e-2_f64 * t20753 + 0.85748036236139473944e-3_f64 * t418 * t386 * t2325 * t4876 + 0.17149607247227894789e-2_f64 * t1180 * t1181 * t1899 * t3169 + 0.16006300097412701803e-1_f64 * t12814 - 0.25724410870841842184e-2_f64 * t1180 * t1165 * t20764 * t3176 + 0.10289764348336736873e-1_f64 * t13591 * t1165 * t1532 * t20769 * t943 - 0.10289764348336736873e-1_f64 * t3462 * t1165 * t1532 * t20775 - 7.0_f64 / 24.0_f64 * t15746;
    t20781
}
