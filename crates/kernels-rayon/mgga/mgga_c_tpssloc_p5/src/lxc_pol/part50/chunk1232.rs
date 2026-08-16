//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1232/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1232(t4017: f64, t79: f64, t8513: f64, t31003: f64, t45844: f64, t12571: f64, t31016: f64, t4021: f64, t8307: f64, t113845: f64, t113848: f64, t113851: f64, t119944: f64, t119948: f64, t119952: f64, t119955: f64, t119965: f64, t119971: f64, t2240: f64, t31004: f64, t31006: f64, t31017: f64, t31019: f64, t31022: f64, t31024: f64, t33107: f64, t33115: f64, t33119: f64, t6504: f64, t8301: f64, t8309: f64) -> f64 {
    let t119975 = t8513 * t79 * t4017;
    let t119978 = t45844 * t31003;
    let t119981 = t12571 * t31016;
    let t119984 = t12571 * t31003;
    let t119990 = t8513 * t8307 * t4021;
    let t119993 = 5.0_f64 / 36.0_f64 * t31017 * t119944 - 5.0_f64 / 12.0_f64 * t31004 * t119948 + 5.0_f64 / 36.0_f64 * t31017 * t119952 + 5.0_f64 / 144.0_f64 * t119955 * t8309 + 5.0_f64 / 72.0_f64 * t113848 * t33115 + 5.0_f64 / 72.0_f64 * t2240 * t8301 * t6504 * t33115 + 5.0_f64 / 72.0_f64 * t31017 * t119965 + 5.0_f64 / 72.0_f64 * t113851 * t33119 + 5.0_f64 / 72.0_f64 * t31022 * t119971 + 5.0_f64 / 72.0_f64 * t31022 * t119975 - 5.0_f64 / 24.0_f64 * t119978 * t31006 + 5.0_f64 / 72.0_f64 * t119981 * t31019 + 5.0_f64 / 72.0_f64 * t119984 * t31024 - 5.0_f64 / 24.0_f64 * t113845 * t33107 - 5.0_f64 / 24.0_f64 * t31004 * t119990;
    t119993
}
