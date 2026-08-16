//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1492/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1492(t31377: f64, t571: f64, t1464: f64, t8372: f64, t2178: f64, t2371: f64, t670: f64, t8273: f64, t31027: f64, t31271: f64, t116929: f64, t8358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117369 = 2.0_f64 * t571 * t31377;
    let t117374 = 2.0_f64 * t8372 * t1464;
    let t117381 = t2371 * t2178;
    let t117385 = t670 * t8273;
    let t117450 = 4.0_f64 / 3.0_f64 * t31027 * t31271;
    let t117457 = t116929 * t8358;
    (t117369, t117374, t117381, t117385, t117450, t117457)
}
