//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1284/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1284(t1140: f64, t6171: f64, t6175: f64, t1805: f64, t3570: f64, t3379: f64, t6237: f64, t3409: f64, t5899: f64, t1096: f64, t1165: f64, t1173: f64, t1181: f64, t18388: f64, t18392: f64, t18396: f64, t18400: f64, t22705: f64, t3396: f64, t4267: f64, t4463: f64, t4533: f64, t4706: f64, t5852: f64) -> f64 {
    let t23751 = t1140 * t6171;
    let t23753 = t1140 * t6175;
    let t23755 = t3570 * t1805;
    let t23765 = t3379 * t6237;
    let t23773 = t3409 * t5899;
    let t23777 = 7.0_f64 / 72.0_f64 * t23751 + 7.0_f64 / 72.0_f64 * t23753 - 35.0_f64 / 216.0_f64 * t23755 + 0.68598428988911579156e-2_f64 * t3396 * t1181 * t22705 * t1096 - 0.17149607247227894789e-1_f64 * t4463 * t1165 * t4267 * t4533 + 0.17149607247227894789e-2_f64 * t23765 + 0.85748036236139473944e-3_f64 * t1173 * t1165 * t5852 * t4706 - 0.10289764348336736873e-1_f64 * t18388 - 0.51448821741683684366e-2_f64 * t18392 - 0.40015750243531754508e-2_f64 * t23773 - 0.25724410870841842183e-2_f64 * t18396 - 0.51448821741683684366e-2_f64 * t18400;
    t23777
}
