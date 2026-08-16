//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 988/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk988(t10676: f64, t11591: f64, t10673: f64, t2810: f64, t3446: f64, t3453: f64, t2813: f64, t3308: f64, t3692: f64, t3429: f64, t2816: f64, t10660: f64, t10678: f64, t10685: f64, t11517: f64, t11521: f64, t11525: f64, t11527: f64, t11530: f64, t11580: f64, t11585: f64, t11589: f64) -> (f64, f64, f64) {
    let t11592 = t11591 * t10676;
    let t11593 = t10673 * t11592;
    let t11598 = t3446 * t3453 * t2810;
    let t11601 = t3446 * t3453 * t2813;
    let t11603 = t3308 * t3692;
    let t11604 = t3429 * t11603;
    let t11607 = t3446 * t3453 * t2816;
    let t11609 = -0.15243824895787514157e-3_f64 * t10660 + 0.96056421943322389208e-3_f64 * t11580 - t11517 + t11521 + 0.36021158228745895953e-3_f64 * t11585 + 0.36021158228745895953e-3_f64 * t11589 - 0.5124043883133942371e-4_f64 * t11593 - 0.51240438831339423711e-4_f64 * t10678 + 0.36021158228745895953e-3_f64 * t10685 - 0.36021158228745895953e-3_f64 * t11598 - 0.36021158228745895953e-3_f64 * t11601 - 0.15243824895787514157e-3_f64 * t11604 - 0.36021158228745895953e-3_f64 * t11607 - t11525 - t11527 + t11530;
    (t11592, t11603, t11609)
}
