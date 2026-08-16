//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 736/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk736(t20348: f64, t9224: f64, t12809: f64, t12852: f64, t17272: f64, t17274: f64, t17276: f64, t17310: f64, t20810: f64, t20813: f64, t20818: f64, t20823: f64, t20827: f64, t462: f64, t92: f64) -> (f64, f64) {
    let t20830 = t9224 * t20348;
    let t20836 = -2.0_f64 * t462 * t20810 + 2.0_f64 * t462 * t20813 - 4.0_f64 / 9.0_f64 * t12852 - t92 * t20818 - 2.0_f64 / 3.0_f64 * t17310 - 4.0_f64 / 3.0_f64 * t12809 - 6.0_f64 * t92 * t20823 + 6.0_f64 * t462 * t20827 - 10.0_f64 / 27.0_f64 * t462 * t20830 - 2.0_f64 / 3.0_f64 * t17272 + t17274 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t17276;
    (t20830, t20836)
}
