//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 975/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk975(t3: f64, t8416: f64, t1518: f64, t8342: f64, t117: f64, t8406: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64, f64) {
    let t8417 = t3 * t8416;
    let t8421 = param_d * t8416;
    let t8427 = t8342 * t1518;
    let t8430 = t117 * t8406;
    let t8433 = 3.0_f64 * t1916 * t2209 + 3.0_f64 * t1918 * t2207 + 6.0_f64 * t572 * t8427 + 3.0_f64 * t572 * t8430 + t573 * t8421;
    (t8417, t8421, t8427, t8430, t8433)
}
