//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1766/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1766(t25222: f64, t857: f64, t2656: f64, t7045: f64, t240: f64, t7036: f64) -> (f64, f64, f64, f64) {
    let t25223 = t25222 * t857;
    let t25224 = 0.16006300097412701803e-1_f64 * t25223;
    let t25225 = t7045 * t2656;
    let t25227 = t7036 * t240;
    (t25223, t25224, t25225, t25227)
}
