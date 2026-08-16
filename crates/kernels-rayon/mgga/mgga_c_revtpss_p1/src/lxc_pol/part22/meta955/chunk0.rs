//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3199/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3199(t10355: f64, t44: f64, t10368: f64, t56: f64, t1518: f64, t670: f64, t1913: f64, t4168: f64, t18217: f64, t571: f64, t1921: f64, t4153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60308 = t44 * t10355;
    let t60311 = t56 * t10368;
    let t60595 = t670 * t1518;
    let t60607 = t1913 * t4168;
    let t60609 = t571 * t18217;
    let t60611 = t4153 * t1921;
    (t60308, t60311, t60595, t60607, t60609, t60611)
}
