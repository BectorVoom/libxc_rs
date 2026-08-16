//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3174/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3174(t11789: f64, t1227: f64, t248: f64, t5975: f64, t18321: f64, t3548: f64, t15437: f64, t15502: f64, t15506: f64, t4965: f64, t5023: f64, t1232: f64, t15498: f64, t15594: f64, t19083: f64, t3511: f64, t3518: f64, t3527: f64, t3531: f64, t44811: f64, t4974: f64, t52575: f64, t52580: f64, t52583: f64, t52586: f64, t52599: f64) -> f64 {
    let t65689 = t1227 * t248 * t11789 * t5975;
    let t65691 = t18321 * t3548;
    let t65703 = t15437 * t15502;
    let t65706 = t15437 * t15506;
    let t65709 = t4965 * t5023;
    let t65716 = t65689 / 10368.0_f64 - 11.0_f64 / 486.0_f64 * t65691 + t15498 * t4974 / 108.0_f64 + t52575 / 162.0_f64 + 5.0_f64 / 10368.0_f64 * t52580 + 5.0_f64 / 1728.0_f64 * t52583 - t52586 / 1152.0_f64 + t44811 / 1296.0_f64 - t15594 * t4974 / 576.0_f64 - t52599 / 243.0_f64 - t65703 * t3511 / 144.0_f64 + t65706 * t3518 / 288.0_f64 + t65709 * t1232 / 216.0_f64 + t19083 * t3527 / 432.0_f64 + t19083 * t3531 / 216.0_f64;
    t65716
}
