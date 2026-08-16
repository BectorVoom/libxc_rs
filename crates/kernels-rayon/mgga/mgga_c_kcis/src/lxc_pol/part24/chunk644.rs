//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 644/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk644(t7589: f64, t7592: f64, t7583: f64, t137: f64, t710: f64, t86: f64, t748: f64, t754: f64, t774: f64) -> (f64, f64, f64, f64, f64) {
    let t7593 = t7589 * t7592;
    let t7595 = t7589 * t7583;
    let t7598 = t86 * t710 * t137;
    let t7601 = t86 * t748 * t137;
    let t7603 = t754 * t774;
    (t7593, t7595, t7598, t7601, t7603)
}
