//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2128/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2128(t22633: f64, t508: f64, t1501: f64, t5883: f64, t10271: f64, t10273: f64, t10275: f64, t10278: f64, t10280: f64, t10282: f64, t10284: f64, t10287: f64, t10289: f64, t10291: f64, t10295: f64) -> (f64, f64, f64) {
    let t22634 = t508 * t22633;
    let t22639 = t1501 * t5883;
    let t22648 = -t10271 - t10273 - t10275 - t10278 - t10280 - t10282 - t10284 - t10287 - t10289 - t10291 - t10295;
    (t22634, t22639, t22648)
}
