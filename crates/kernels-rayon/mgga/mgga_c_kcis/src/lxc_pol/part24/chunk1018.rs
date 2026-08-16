//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1018/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1018(t26519: f64, t26653: f64, t180: f64, t7671: f64, t838: f64, t380: f64, t982: f64) -> (f64, f64, f64, f64) {
    let t26654 = t26519 + t26653;
    let t26655 = t180 * t26654;
    let t26656 = t838 * t7671;
    let t26657 = 2.0_f64 * t26656;
    let t26671 = t380 * t982;
    (t26654, t26655, t26657, t26671)
}
