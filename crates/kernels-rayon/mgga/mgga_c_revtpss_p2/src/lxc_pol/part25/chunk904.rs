//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 904/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk904(t10697: f64, t72: f64, t10627: f64, t828: f64, t136: f64, t2476: f64, t221: f64, t2394: f64, t2674: f64, t231: f64, t243: f64, t2645: f64) -> (f64, f64, f64, f64) {
    let t10698 = t10697 * t72;
    let t10700 = t10698 * t828 * t10627;
    let t10703 = t2476 * t136;
    let t10705 = t10703 * t221 * t2394;
    let t10706 = t2674 * t10705;
    let t10709 = t243 * t2645 * t231;
    (t10700, t10705, t10706, t10709)
}
