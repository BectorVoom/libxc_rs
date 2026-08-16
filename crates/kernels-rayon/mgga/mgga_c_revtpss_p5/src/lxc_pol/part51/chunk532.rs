//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 532/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk532(t1579: f64, t886: f64, t2770: f64, t1558: f64, t251: f64, t231: f64, t2783: f64, t2782: f64, t1559: f64, t72: f64, t686: f64, t2798: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4486 = t1579 * t886;
    let t4487 = t2770 * t4486;
    let t4494 = t251 * t1558;
    let t4496 = t2783 * t4494 * t231;
    let t4497 = t2782 * t4496;
    let t4499 = t1559 * t72;
    let t4500 = t4499 * t686;
    let t4501 = t2798 * t4500;
    (t4486, t4487, t4494, t4497, t4500, t4501)
}
