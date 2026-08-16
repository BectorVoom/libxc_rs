//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 507/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk507(t2219: f64, t2223: f64, t2226: f64, t2230: f64, t2233: f64, t2239: f64, t1466: f64, t602: f64, t1497: f64, t644: f64, t1469: f64, t606: f64) -> (f64, f64, f64, f64) {
    let t4171 = -t2219 + t2223 - t2226 + t2230 - t2233 + t2239;
    let t4173 = t1466 * t602;
    let t4178 = t1497 * t644;
    let t4181 = t606 * t1469;
    (t4171, t4173, t4178, t4181)
}
