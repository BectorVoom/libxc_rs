//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 647/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk647(t322: f64, t3357: f64, t3368: f64, t3625: f64, t3627: f64, t3630: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t3632 = t3357 + t3625 / 8.0_f64 - t3627 / 8.0_f64 + t3630 / 4.0_f64 + t3368;
    let t3633 = piecewise3(t324, 0.0_f64, t3632);
    (t3632, t3633)
}
