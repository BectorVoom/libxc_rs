//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1240/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1240(t1163: f64, t1181: f64, t5122: f64, t5862: f64, t1165: f64, t4210: f64, t1772: f64, t435: f64, t5752: f64, t943: f64, t1899: f64, t1180: f64, t13627: f64, t13631: f64, t13636: f64, t13638: f64, t1531: f64, t1532: f64, t1552: f64, t17371: f64, t1759: f64, t3176: f64, t3403: f64, t5867: f64, t922: f64) -> (f64, f64, f64) {
    let t22685 = t1163 * t1181 * t5862 * t5122;
    let t22693 = t1163 * t1165 * t5862 * t4210;
    let t22705 = t435 * t1772;
    let t22710 = t5752 * t943;
    let t22717 = t1163 * t1165 * t1899 * t4210;
    let t22719 = 0.85748036236139473944e-3_f64 * t22685 + 0.85748036236139473944e-3_f64 * t1180 * t1165 * t5867 * t3176 + 0.42874018118069736972e-3_f64 * t22693 + 0.80031500487063509016e-2_f64 * t13627 + 0.17149607247227894789e-1_f64 * t3403 * t1165 * t1552 * t1759 * t922 + 0.17149607247227894789e-2_f64 * t13631 + 0.75585306015559980738e-1_f64 * t13636 + 0.13605355082800796533e0_f64 * t13638 + 0.17149607247227894789e-2_f64 * t17371 + 0.17149607247227894789e-2_f64 * t1180 * t1181 * t22705 * t3176 + 0.51448821741683684367e-2_f64 * t1531 * t1181 * t1532 * t22710 - 0.85748036236139473944e-3_f64 * t22717;
    (t22705, t22710, t22719)
}
