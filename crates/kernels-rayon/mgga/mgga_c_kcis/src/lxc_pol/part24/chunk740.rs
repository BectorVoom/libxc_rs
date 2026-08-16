//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 740/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk740(t2545: f64, t2558: f64, t2551: f64, t8546: f64, t22: f64, t728: f64, t736: f64, t126: f64, t2379: f64, t15: f64, t684: f64, t762: f64) -> (f64, f64, f64, f64, f64) {
    let t9113 = t2545 * t2558;
    let t9118 = t2551 * t8546;
    let t9120 = t22 * t736 * t728;
    let t9123 = t2379 * t126;
    let t9124 = t9123 * t15;
    let t9129 = t684 * t762;
    (t9113, t9118, t9120, t9124, t9129)
}
