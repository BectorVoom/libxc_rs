//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1406/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1406(t1888: f64, t23270: f64, t26729: f64, t113038: f64, t113045: f64, t118924: f64, t118928: f64, t13053: f64, t13463: f64, t25168: f64, t26582: f64, t26713: f64, t31343: f64, t31409: f64, t33405: f64, t4268: f64, t6627: f64, t6632: f64, t7516: f64, t8553: f64, t87013: f64, t92981: f64) -> f64 {
    let t121745 = t1888 * t23270 * t26729;
    let t121747 = -6.0_f64 * t87013 * t33405 + t113038 + 2.0_f64 * t6627 * t26582 + 2.0_f64 * t13463 * t8553 + 2.0_f64 * t4268 * t31343 + 2.0_f64 * t4268 * t31409 - t118924 + 2.0_f64 * t26713 * t6632 + 2.0_f64 * t13053 * t8553 - 6.0_f64 * t25168 * t92981 * t7516 - t113045 - 0.49348022005446793095e-1_f64 * t121745 + t118928;
    t121747
}
