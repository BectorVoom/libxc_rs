//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 699/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk699(t2724: f64, t827: f64, t828: f64, t159: f64, t243: f64, t216: f64, t124: f64, t2394: f64, t800: f64, t2712: f64, t785: f64) -> (f64, f64, f64, f64, f64) {
    let t2726 = t827 * t828 * t2724;
    let t2729 = t159 * t243;
    let t2730 = t216 * t2729;
    let t2731 = t124 * t2394;
    let t2732 = t800 * t2731;
    let t2735 = t2712 * t785;
    (t2726, t2729, t2730, t2732, t2735)
}
