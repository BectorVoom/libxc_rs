//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 716/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk716(t1821: f64, t712: f64, t1691: f64, t188: f64, t1906: f64, t1647: f64, t652: f64, t390: f64, t631: f64, t644: f64, t1659: f64, t189: f64) -> (f64, f64, f64, f64) {
    let t5593 = t712 * t1821;
    let t5594 = t5593 * t1691;
    let t5597 = t1906 * t188;
    let t5598 = t652 * t1647;
    let t5599 = t5597 * t5598;
    let t5601 = 0.51550785283058921156e1_f64 * t390 * t5599;
    let t5602 = t631 * t644;
    let t5603 = t5602 * t1659;
    let t5605 = 0.21369999999999999999e0_f64 * t390 * t5603;
    let t5606 = t189 * t1647;
    (t5594, t5601, t5605, t5606)
}
