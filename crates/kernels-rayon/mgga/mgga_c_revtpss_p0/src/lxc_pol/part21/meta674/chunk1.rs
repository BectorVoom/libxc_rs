//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2476/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2476(t2439: f64, t3421: f64, t12278: f64, t698: f64, t12274: f64, t25273: f64, t268: f64, t404: f64) -> (f64, f64, f64, f64) {
    let t43783 = t2439 * t3421;
    let t43785 = t698 * t12278;
    let t43787 = t698 * t12274;
    let t43813 = t268 * t25273 * t404;
    (t43783, t43785, t43787, t43813)
}
