//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1607/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1607(t2: f64, t3833: f64, t1711: f64, t9350: f64, t3841: f64, t1857: f64, t3857: f64, t177: f64, t5566: f64, t762: f64, t1450: f64, t5778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13553 = t3833 * t2;
    let t13565 = t9350 * t1711;
    let t13568 = t3841 * t2;
    let t13584 = t3857 * t1857;
    let t13597 = t5566 * t177;
    let t13599 = 0.11696447245269292414e1_f64 * t13597 * t762;
    let t13600 = t5778 * t1450;
    (t13553, t13565, t13568, t13584, t13597, t13599, t13600)
}
