//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2000/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2000(t3201: f64, t7126: f64, t7114: f64, t1024: f64, t25576: f64, t11997: f64, t3141: f64, t7120: f64, t11858: f64, t27492: f64, t11926: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93618 = t7126 * t3201;
    let t93622 = t7114 * t3201;
    let t93646 = t1024 * t25576;
    let t93655 = t3141 * t7120 * t11997;
    let t93658 = t11858 * t27492;
    let t93667 = t11926 * t25516;
    (t93618, t93622, t93646, t93655, t93658, t93667)
}
