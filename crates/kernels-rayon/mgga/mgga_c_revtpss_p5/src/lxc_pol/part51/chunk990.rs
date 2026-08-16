//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 990/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk990(t1568: f64, t8477: f64, t1579: f64, t8471: f64, t31812: f64, t1558: f64, t231: f64, t31817: f64, t1949: f64, t7759: f64, t8650: f64, t8485: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33695 = t8477 * t1568;
    let t33698 = t8471 * t1579;
    let t33699 = t31812 * t33698;
    let t33703 = t8471 * t1558 * t231;
    let t33704 = t31817 * t33703;
    let t33707 = t1949 * t7759;
    let t33708 = t8650 * t33707;
    let t33711 = t33695 * t8485;
    (t33695, t33698, t33699, t33703, t33704, t33707, t33708, t33711)
}
