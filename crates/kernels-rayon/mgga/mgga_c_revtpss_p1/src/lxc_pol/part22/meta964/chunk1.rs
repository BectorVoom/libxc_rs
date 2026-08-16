//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3227/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3227(t15071: f64, t1940: f64, t2403: f64, t40084: f64, t40088: f64, t40099: f64, t40103: f64, t40115: f64, t4556: f64, t61197: f64, t61198: f64, t61199: f64, t61200: f64, t61202: f64, t61203: f64, t61209: f64) -> f64 {
    let t61210 = -2.0_f64 * t15071 * t1940 * t4556 - 6.0_f64 * t2403 * t4556 * t61203 + t40084 + t40088 + t40099 + t40103 - t40115 + t61197 - t61198 + t61199 + t61200 + t61202 + t61209;
    t61210
}
