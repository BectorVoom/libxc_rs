//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1338/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1338(t39501: f64, t871: f64, t10115: f64, t225: f64, t10866: f64, t232: f64, t235: f64, t239: f64, t820: f64, t2723: f64, t2482: f64, t2719: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40316 = 0.56911289235245161963e-1_f64 * t39501 * t871;
    let t40317 = t10115 * t225;
    let t40321 = 1.0_f64 / t10866 / t232;
    let t40322 = t40321 * t235;
    let t40324 = t820 * t40322 * t239;
    let t40325 = t2723 * t2723;
    let t40336 = t2482 * t2719 * t596;
    (t40316, t40317, t40321, t40324, t40325, t40336)
}
