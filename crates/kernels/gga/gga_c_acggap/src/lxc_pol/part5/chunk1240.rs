//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1240/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1240<F: Float>(t1163: F, t1181: F, t5122: F, t5862: F, t1165: F, t4210: F, t1772: F, t435: F, t5752: F, t943: F, t1899: F, t1180: F, t13627: F, t13631: F, t13636: F, t13638: F, t1531: F, t1532: F, t1552: F, t17371: F, t1759: F, t3176: F, t3403: F, t5867: F, t922: F) -> (F, F, F) {
    let t22685 = t1163 * t1181 * t5862 * t5122;
    let t22693 = t1163 * t1165 * t5862 * t4210;
    let t22705 = t435 * t1772;
    let t22710 = t5752 * t943;
    let t22717 = t1163 * t1165 * t1899 * t4210;
    let t22719 = F::new(0.85748036236139473944e-3) * t22685 + F::new(0.85748036236139473944e-3) * t1180 * t1165 * t5867 * t3176 + F::new(0.42874018118069736972e-3) * t22693 + F::new(0.80031500487063509016e-2) * t13627 + F::new(0.17149607247227894789e-1) * t3403 * t1165 * t1552 * t1759 * t922 + F::new(0.17149607247227894789e-2) * t13631 + F::new(0.75585306015559980738e-1) * t13636 + F::new(0.13605355082800796533e0) * t13638 + F::new(0.17149607247227894789e-2) * t17371 + F::new(0.17149607247227894789e-2) * t1180 * t1181 * t22705 * t3176 + F::new(0.51448821741683684367e-2) * t1531 * t1181 * t1532 * t22710 - F::new(0.85748036236139473944e-3) * t22717;
    (t22705, t22710, t22719)
}
