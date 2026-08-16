//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1029/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1029(t1399: f64, t32195: f64, t5673: f64, t32194: f64, t1955: f64, t843: f64, t8571: f64, t8575: f64, t239: f64, t240: f64, t31752: f64, t545: f64) -> (f64, f64, f64, f64) {
    let t32197 = t5673 * t32195 * t1399;
    let t32198 = t32194 * t32197;
    let t32202 = t1955 * t8571 * t843 * t8575;
    let t32206 = t31752 * t545 * t239 * t240;
    (t32197, t32198, t32202, t32206)
}
