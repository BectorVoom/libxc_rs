//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1185/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1185(t95585: f64, t27769: f64, t2861: f64, t27815: f64, t7703: f64, t9938: f64, t14443: f64, t27821: f64, t14570: f64, t283: f64, t990: f64, t9588: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95586 = 0.66327777777777777776e-2_f64 * t95585;
    let t95587 = t2861 * t27769;
    let t95605 = 0.15445601851851851852e-3_f64 * t7703 * t9938 * t27815;
    let t95606 = t14443 * t27821;
    let t95608 = 0.15445601851851851852e-3_f64 * t7703 * t95606;
    let t95640 = t14570 * t283 * t990;
    let t95655 = t9588 * t283;
    (t95586, t95587, t95605, t95606, t95608, t95640, t95655)
}
