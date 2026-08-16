//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 551/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk551(t487: f64, t5219: f64, t1770: f64, t1209: f64, t1811: f64, t1256: f64, t1804: f64, t1786: f64, t1796: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5220 = t5219 * t487;
    let t5225 = t1770 * t487;
    let t5251 = t1209 * t1811;
    let t5254 = t1804 * t1256;
    let t5256 = t1786 * t1256;
    let t5265 = t3172 * t1796;
    (t5220, t5225, t5251, t5254, t5256, t5265)
}
