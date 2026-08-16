//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1242/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1242(t1454: f64, t372: f64, t1131: f64, t1143: f64, t1165: f64, t1181: f64, t13654: f64, t13851: f64, t1501: f64, t1532: f64, t1734: f64, t17411: f64, t17421: f64, t17430: f64, t17436: f64, t17441: f64, t1748: f64, t1894: f64, t335: f64, t336: f64, t3396: f64, t3565: f64, t367: f64, t4099: f64, t4876: f64, t513: f64, t5506: f64, t6138: f64, t922: f64) -> (f64, f64) {
    let t22778 = t1454 * t372;
    let t22787 = 0.45351183609335988442e0_f64 * t17411 + 0.17149607247227894789e-2_f64 * t17421 - t335 * t336 * t1143 * t5506 / 24.0_f64 - t367 * t336 * t1894 * t1131 / 96.0_f64 - t367 * t336 * t4876 * t513 / 48.0_f64 - t335 * t336 * t1501 * t4099 / 24.0_f64 - t335 * t336 * t3565 * t1734 / 48.0_f64 + 0.51448821741683684367e-1_f64 * t13851 * t1165 * t1532 * t1748 * t922 - 0.41159057393346947493e-1_f64 * t3396 * t1181 * t6138 * t22778 + 0.68598428988911579156e-2_f64 * t17430 + 35.0_f64 / 108.0_f64 * t13654 - 0.17149607247227894789e-2_f64 * t17436 - 0.17149607247227894789e-2_f64 * t17441;
    (t22778, t22787)
}
