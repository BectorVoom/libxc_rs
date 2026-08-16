//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1067/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1067(t26519: f64, t26653: f64, t180: f64, t7671: f64, t838: f64, t2209: f64, t2802: f64, t233: f64, t7684: f64, t911: f64, t7827: f64, t915: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26654 = t26519 + t26653;
    let t26655 = t180 * t26654;
    let t26656 = t838 * t7671;
    let t26658 = t2802 * t2209;
    let t26659 = t233 * t26658;
    let t26660 = t26659 / 16.0_f64;
    let t26662 = t911 * t7684;
    let t26663 = t26662 / 8.0_f64;
    let t26664 = t915 * t7827;
    (t26654, t26655, t26656, t26660, t26663, t26664)
}
