//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1844/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1844(t86928: f64, t86940: f64, t86942: f64, t86950: f64, t86967: f64, t225: f64, t26708: f64, t87028: f64, t87066: f64, t87100: f64, t87153: f64, t87165: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92415 = 0.16449340668482264365e-1_f64 * t86928;
    let t92425 = 0.16449340668482264365e-1_f64 * t86940;
    let t92426 = 0.76763589786250567036e-1_f64 * t86942;
    let t92431 = 0.15352717957250113407e0_f64 * t86950;
    let t92434 = 0.15352717957250113407e0_f64 * t86967;
    let t92439 = t26708 * t225;
    let t92486 = 0.3289868133696452873e-1_f64 * t87028;
    let t92491 = 0.76763589786250567036e-1_f64 * t87066;
    let t92502 = 0.16449340668482264365e-1_f64 * t87100;
    let t92515 = 0.16449340668482264365e-1_f64 * t87153;
    let t92530 = 0.3289868133696452873e-1_f64 * t87165;
    (t92415, t92425, t92426, t92431, t92434, t92439, t92486, t92491, t92502, t92515, t92530)
}
