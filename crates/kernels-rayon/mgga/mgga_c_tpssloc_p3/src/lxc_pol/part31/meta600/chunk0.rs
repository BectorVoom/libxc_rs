//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1845/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1845(t26653: f64, t814: f64, t87520: f64, t87522: f64, t87533: f64, t87535: f64, t87544: f64, t87546: f64, t87197: f64, t87205: f64, t87211: f64, t87233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92546 = t814 * t26653;
    let t92551 = 0.3289868133696452873e-1_f64 * t87520;
    let t92556 = 0.15352717957250113407e0_f64 * t87522;
    let t92560 = 0.15352717957250113407e0_f64 * t87533;
    let t92561 = 0.76763589786250567036e-1_f64 * t87535;
    let t92564 = 0.3289868133696452873e-1_f64 * t87544;
    let t92565 = 0.15352717957250113407e0_f64 * t87546;
    let t92578 = 7.0_f64 / 144.0_f64 * t87197;
    let t92580 = 0.56521858531796547194e-2_f64 * t87205;
    let t92582 = 0.13457585364713463618e-3_f64 * t87211;
    let t92590 = 0.26915170729426927236e-3_f64 * t87233;
    (t92546, t92551, t92556, t92560, t92561, t92564, t92565, t92578, t92580, t92582, t92590)
}
