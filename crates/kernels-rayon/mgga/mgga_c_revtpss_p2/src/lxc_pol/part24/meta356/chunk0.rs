//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1222/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1222(t1012: f64, t24016: f64, t23598: f64, t373: f64, t371: f64, t372: f64, t1651: f64, t6244: f64) -> (f64, f64, f64, f64) {
    let t24017 = t1012 * t24016;
    let t24022 = t373 * t23598;
    let t24024 = t371 * t372 * t24022;
    let t24031 = t6244 * t1651;
    (t24017, t24022, t24024, t24031)
}
