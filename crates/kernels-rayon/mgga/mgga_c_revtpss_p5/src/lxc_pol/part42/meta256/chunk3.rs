//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 981/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk981(t1518: f64, t8342: f64, t117: f64, t8406: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, t8421: f64, t587: f64, t65: f64) -> (f64, f64, f64, f64) {
    let t8427 = t8342 * t1518;
    let t8430 = t117 * t8406;
    let t8433 = 3.0_f64 * t1916 * t2209 + 3.0_f64 * t1918 * t2207 + 6.0_f64 * t572 * t8427 + 3.0_f64 * t572 * t8430 + t573 * t8421;
    let t8779 = 1.0_f64 / t65 / t587;
    (t8427, t8430, t8433, t8779)
}
