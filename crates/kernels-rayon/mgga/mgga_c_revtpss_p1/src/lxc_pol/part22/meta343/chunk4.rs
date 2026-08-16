//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1822/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1822(t11132: f64, t11337: f64, t3010: f64, t963: f64) -> (f64, f64, f64) {
    let t11479 = 0.93932222222222222223e0_f64 * t11132;
    let t11480 = 0.36793333333333333333e0_f64 * t11337;
    let t11506 = 1.0_f64 / t3010 / t963;
    (t11479, t11480, t11506)
}
