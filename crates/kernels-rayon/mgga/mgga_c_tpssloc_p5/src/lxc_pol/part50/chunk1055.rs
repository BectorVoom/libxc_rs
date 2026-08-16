//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1055/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1055(t5: f64, t2240: f64, t31016: f64, t6504: f64, t8307: f64, t8513: f64, t31003: f64, t641: f64, t79: f64, t31000: f64, t31004: f64, t31006: f64, t31010: f64, t31013: f64, t8309: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t31017 = t2240 * t31016;
    let t31019 = t8513 * t8307 * t6504;
    let t31022 = t2240 * t31003;
    let t31024 = t8513 * t79 * t641;
    let t31028 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t31000 * t8309 - 5.0_f64 / 24.0_f64 * t31004 * t31006 - 5.0_f64 / 36.0_f64 * t31010 * t31013 + 5.0_f64 / 72.0_f64 * t31017 * t31019 + 5.0_f64 / 72.0_f64 * t31022 * t31024);
    (t31017, t31019, t31022, t31024, t31028)
}
