//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 734/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk734(t20031: f64, t3499: f64, t20556: f64, t2102: f64, t20560: f64, t20336: f64, t582: f64, t17279: f64, t17281: f64, t20786: f64, t20789: f64, t20793: f64, t20796: f64, t462: f64, t9178: f64) -> (f64, f64, f64, f64, f64) {
    let t20799 = t3499 * t20031;
    let t20802 = t2102 * t20556;
    let t20804 = t2102 * t20560;
    let t20806 = t582 * t20336;
    let t20809 = -2.0_f64 * t462 * t20786 - 2.0_f64 * t462 * t20789 - t9178 + t17279 - 2.0_f64 * t17281 + 2.0_f64 / 3.0_f64 * t462 * t20793 + 4.0_f64 / 3.0_f64 * t462 * t20796 - 2.0_f64 / 3.0_f64 * t462 * t20799 + t462 * t20802 + t462 * t20804 - t462 * t20806 / 3.0_f64;
    (t20799, t20802, t20804, t20806, t20809)
}
