//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2007/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2007(t25610: f64, t27668: f64, t25460: f64, t3057: f64, t25698: f64, t378: f64, t8521: f64, t11108: f64, t7177: f64, t1989: f64, t41937: f64, t1113: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94085 = t25610 * t27668;
    let t94095 = t3057 * t25460;
    let t94121 = t25698 * t378;
    let t94122 = t94121 * t8521;
    let t94142 = t7177 * t11108;
    let t94149 = t1989 * t41937;
    let t94245 = t2411 * t1113;
    (t94085, t94095, t94122, t94142, t94149, t94245)
}
