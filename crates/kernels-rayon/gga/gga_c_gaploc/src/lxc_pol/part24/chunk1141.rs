//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1141/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1141(t587: f64, t589: f64, t9278: f64, t1407: f64, t9548: f64, t20887: f64, t9305: f64, t21417: f64, t1397: f64, t6603: f64, t9287: f64, t1415: f64, t6699: f64, t7030: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30606 = t587 * t589 * t9278;
    let t30607 = 0.1022478025437886658e1_f64 * t30606;
    let t30629 = 0.17041300423964777634e0_f64 * t1407 * t9548;
    let t30631 = 0.29792074959875355558e-1_f64 * t9305 * t20887;
    let t30633 = 0.11916829983950142223e0_f64 * t9305 * t21417;
    let t30642 = t1397 * t6603;
    let t30644 = 0.59584149919750711116e-1_f64 * t30642 * t9287;
    let t30647 = 0.59584149919750711116e-1_f64 * t1415 * t6699 * t7030;
    (t30607, t30629, t30631, t30633, t30642, t30644, t30647)
}
