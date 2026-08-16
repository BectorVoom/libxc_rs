//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1262/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1262(t1014: f64, t28966: f64, t19811: f64, t7718: f64, t9370: f64, t100636: f64, t100641: f64, t28190: f64, t28211: f64, t8087: f64, t93016: f64, t96068: f64, t97212: f64, t97248: f64, t97250: f64, t97267: f64) -> (f64, f64, f64) {
    let t100643 = t1014 * t28966;
    let t100646 = t9370 * t7718 * t19811;
    let t100652 = 0.20594135802469135803e-3_f64 * t97212 + 0.46429444444444444443e-2_f64 * t100636 - 0.41270617283950617283e-2_f64 * t96068 - 0.20594135802469135803e-3_f64 * t93016 - 0.61905925925925925925e-2_f64 * t100641 + 0.15476481481481481481e-2_f64 * t100643 + 0.51588271604938271605e-2_f64 * t100646 - t97248 - t97250 + 0.69505208333333333334e-3_f64 * t28190 * t28211 + 0.69505208333333333334e-3_f64 * t97267 * t8087;
    (t100643, t100646, t100652)
}
