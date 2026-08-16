//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1287/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1287(t1861: f64, t3228: f64, t1008: f64, t5956: f64, t13084: f64, t5737: f64, t1165: f64, t1173: f64, t1180: f64, t1181: f64, t14243: f64, t14245: f64, t1426: f64, t14260: f64, t1532: f64, t1552: f64, t175: f64, t18460: f64, t20433: f64, t21677: f64, t22048: f64, t3169: f64, t3196: f64, t418: f64, t5852: f64) -> f64 {
    let t23864 = t3228 * t1861;
    let t23866 = t1008 * t5956;
    let t23872 = t13084 * t5737;
    let t23886 = 0.60023625365297631762e-2_f64 * t18460 - 0.40015750243531754508e-1_f64 * t14243 + 0.12862205435420921092e-2_f64 * t14245 + t14260 + 0.85748036236139473944e-2_f64 * t418 * t1426 * t175 * t21677 - 0.34299214494455789578e-2_f64 * t23864 - 0.68598428988911579156e-2_f64 * t23866 - 0.85748036236139473944e-3_f64 * t1180 * t1181 * t1532 * t22048 + 0.80031500487063509016e-1_f64 * t23872 + 0.85748036236139473944e-3_f64 * t1180 * t1165 * t1552 * t20433 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t5852 * t3196 - 0.85748036236139473944e-3_f64 * t1180 * t1181 * t5852 * t3169;
    t23886
}
