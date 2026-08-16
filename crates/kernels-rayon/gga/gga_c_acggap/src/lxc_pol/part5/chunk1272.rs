//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1272/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1272(t1111: f64, t1165: f64, t20545: f64, t3391: f64, t1090: f64, t1150: f64, t1181: f64, t12473: f64, t1713: f64, t1782: f64, t18097: f64, t18103: f64, t18105: f64, t18107: f64, t18109: f64, t18111: f64, t18119: f64, t1889: f64, t336: f64, t3565: f64, t367: f64, t4417: f64, t4463: f64, t4735: f64, t4757: f64) -> f64 {
    let t23511 = t3391 * t1165 * t20545 * t1111;
    let t23529 = 35.0_f64 / 54.0_f64 * t18097 + 0.10289764348336736874e-1_f64 * t4735 * t1181 * t4417 * t4757 + 0.16006300097412701803e-1_f64 * t18103 + 0.80031500487063509016e-2_f64 * t18105 + 0.10289764348336736874e-1_f64 * t23511 + 0.80031500487063509016e-2_f64 * t18107 + 0.40015750243531754508e-2_f64 * t18109 + 0.68598428988911579156e-2_f64 * t18111 + 0.17149607247227894789e-1_f64 * t4463 * t1181 * t1889 * t1090 + 0.68598428988911579156e-2_f64 * t18119 + t367 * t336 * t12473 * t1782 / 48.0_f64 + t1150 * t336 * t3565 * t1713 / 16.0_f64;
    t23529
}
