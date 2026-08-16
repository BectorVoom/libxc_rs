//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 541/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk541(t1785: f64, t480: f64, t1774: f64, t482: f64, t372: f64, t371: f64) -> (f64, f64, f64) {
    let t1786 = t1785 * t480;
    let t1789 = t482 * t1774;
    let t1790 = t372 * t1789;
    let t1791 = t371 * t1790;
    (t1786, t1789, t1791)
}
