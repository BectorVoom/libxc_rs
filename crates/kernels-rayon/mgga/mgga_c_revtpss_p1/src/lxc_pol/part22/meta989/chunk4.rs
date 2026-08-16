//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3364/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3364(t42518: f64, t52011: f64, t60927: f64, t4606: f64, t2897: f64, t51957: f64, t52110: f64) -> (f64, f64, f64, f64) {
    let t63393 = t52011 * t42518 * t60927;
    let t63395 = t4606 * t4606;
    let t63396 = t2897 * t63395;
    let t63399 = t51957 * t52110 * t60927;
    (t63393, t63395, t63396, t63399)
}
