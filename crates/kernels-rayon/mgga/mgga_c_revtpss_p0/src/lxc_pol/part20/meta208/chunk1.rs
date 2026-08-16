//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 983/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk983(t10290: f64, t2236: f64, t3: f64, t25: f64, t10271: f64, t10273: f64, t10275: f64, t10278: f64, t10280: f64, t10282: f64, t10284: f64, t10287: f64, t10289: f64) -> (f64, f64, f64) {
    let t10291 = 756.0_f64 * t10290;
    let t10292 = t2236 * t3;
    let t10293 = 1.0_f64 / t10292;
    let t10295 = 336.0_f64 * t25 * t10293;
    let t10296 = -t10271 + t10273 - t10275 + t10278 - t10280 + t10282 - t10284 + t10287 - t10289 + t10291 - t10295;
    (t10292, t10293, t10296)
}
