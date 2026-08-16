//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 830/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk830(t2465: f64, t4481: f64, t1579: f64, t886: f64, t2770: f64, t1558: f64, t251: f64, t231: f64, t2783: f64, t2782: f64, t1559: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4482 = t2465 * t4481;
    let t4486 = t1579 * t886;
    let t4487 = t2770 * t4486;
    let t4494 = t251 * t1558;
    let t4496 = t2783 * t4494 * t231;
    let t4497 = t2782 * t4496;
    let t4499 = t1559 * t72;
    (t4482, t4487, t4494, t4496, t4497, t4499)
}
