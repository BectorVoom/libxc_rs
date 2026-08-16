//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3152/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3152(t12702: f64, t17350: f64, t1263: f64, t372: f64, t5284: f64, t13148: f64, t56878: f64, t17728: f64, t460: f64, t489: f64, t17261: f64, t17373: f64) -> (f64, f64, f64, f64, f64) {
    let t56977 = t12702 * t17350;
    let t56981 = t372 * t1263 * t5284;
    let t56997 = t13148 * t56878;
    let t57005 = t460 * t489 * t17728;
    let t57021 = t17261 * t17373;
    (t56977, t56981, t56997, t57005, t57021)
}
