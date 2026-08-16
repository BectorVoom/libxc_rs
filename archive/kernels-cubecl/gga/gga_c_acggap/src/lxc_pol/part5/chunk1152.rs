//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1152/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1152<F: Float>(t2937: F, t495: F, t5746: F, t943: F, t1026: F, t1027: F, t1165: F, t1180: F, t1181: F, t12814: F, t13591: F, t1532: F, t1574: F, t15746: F, t1894: F, t1899: F, t20737: F, t20739: F, t20753: F, t20764: F, t2325: F, t3169: F, t3176: F, t3462: F, t386: F, t418: F, t4847: F, t4876: F, t5679: F, t6119: F) -> F {
    let t20769 = t2937 * t495;
    let t20775 = t5746 * t943;
    let t20781 = F::cast_from(0.85748036236139473944e-3_f64) * t20737 + F::cast_from(0.85748036236139473944e-3_f64) * t20739 + F::cast_from(0.42874018118069736972e-3_f64) * t418 * t386 * t4847 * t1894 + F::cast_from(0.85748036236139473944e-3_f64) * t418 * t386 * t1574 * t6119 + F::cast_from(0.85748036236139473945e-2_f64) * t418 * t1026 * t5679 * t1027 + F::cast_from(0.17149607247227894789e-2_f64) * t20753 + F::cast_from(0.85748036236139473944e-3_f64) * t418 * t386 * t2325 * t4876 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t1181 * t1899 * t3169 + F::cast_from(0.16006300097412701803e-1_f64) * t12814 - F::cast_from(0.25724410870841842184e-2_f64) * t1180 * t1165 * t20764 * t3176 + F::cast_from(0.10289764348336736873e-1_f64) * t13591 * t1165 * t1532 * t20769 * t943 - F::cast_from(0.10289764348336736873e-1_f64) * t3462 * t1165 * t1532 * t20775 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t15746;
    t20781
}
