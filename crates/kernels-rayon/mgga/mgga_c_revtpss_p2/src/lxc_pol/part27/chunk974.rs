//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 974/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk974(t3111: f64, t3188: f64, t3075: f64, t999: f64, t247: f64, t3116: f64, t11173: f64, t373: f64, t371: f64, t372: f64, t3211: f64, t3215: f64) -> (f64, f64, f64, f64, f64) {
    let t11802 = t3188 * t3111;
    let t11804 = t3075 * t999;
    let t11806 = t247 * t3116 * t11804;
    let t11809 = t373 * t11173;
    let t11811 = t371 * t372 * t11809;
    let t11814 = t3211 * t3215;
    (t11802, t11804, t11806, t11811, t11814)
}
