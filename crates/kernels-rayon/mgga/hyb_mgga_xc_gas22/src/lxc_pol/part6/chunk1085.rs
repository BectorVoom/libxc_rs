//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1085/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1085(t10571: f64, t791: f64, t10534: f64, t10549: f64, t6530: f64, t6592: f64, t8676: f64, t8681: f64, t789: f64, t4117: f64, t6601: f64, t3329: f64, t3335: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10572 = t10571 * t791;
    let t10577 = -t6592 + 4.0_f64 / 9.0_f64 * t6530 + 8.0_f64 / 9.0_f64 * t8676 - t8681 - t10534 / 3.0_f64 + t10549;
    let t10578 = t789 * t10577;
    let t10584 = t6601 * t4117;
    let t10585 = t10584 * t791;
    let t10587 = t3335 * t3329;
    (t10572, t10577, t10578, t10584, t10585, t10587)
}
