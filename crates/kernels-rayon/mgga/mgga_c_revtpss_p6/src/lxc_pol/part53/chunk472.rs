//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 472/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk472(t2703: f64, t802: f64, t234: f64, t2453: f64, t595: f64, t65: f64, t235: f64, t826: f64, t232: f64, t821: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2704 = t2703 * t802;
    let t2710 = t2453 * t234;
    let t2712 = 1.0_f64 / t65 / t595;
    let t2713 = t235 * t2712;
    let t2716 = 0.45178982497454656791e-5_f64 * t2710 * t2713 * t826;
    let t2718 = 1.0_f64 / t821 / t232;
    (t2704, t2710, t2712, t2713, t2716, t2718)
}
