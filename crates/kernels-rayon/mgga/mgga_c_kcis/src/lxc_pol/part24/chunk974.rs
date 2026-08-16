//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 974/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk974(t11072: f64, t20579: f64, t330: f64, t6837: f64, t829: f64, t3515: f64, t25: f64, t6775: f64, t1251: f64, t1262: f64, t6330: f64, t11063: f64, t11086: f64, t11093: f64, t11100: f64, t20564: f64, t20570: f64, t20574: f64, t3490: f64, t3514: f64, t6763: f64, t6776: f64) -> f64 {
    let t20580 = t11072 * t20579;
    let t20583 = t6837 * t330;
    let t20584 = t20583 * t829;
    let t20585 = t3515 * t20584;
    let t20590 = t25 * t6775;
    let t20591 = t1251 * t20590;
    let t20593 = t6330 * t1262;
    let t20594 = t3515 * t20593;
    let t20598 = -t3514 * t20564 / 144.0_f64 + t11086 * t6763 / 108.0_f64 - t20570 / 864.0_f64 + t3514 * t20574 / 144.0_f64 - t11063 / 2592.0_f64 + t3514 * t20580 / 288.0_f64 - t3514 * t20585 / 576.0_f64 - t3490 * t6776 / 36.0_f64 + t20591 / 288.0_f64 + t3514 * t20594 / 288.0_f64 + t11093 + t11100 / 324.0_f64;
    t20598
}
