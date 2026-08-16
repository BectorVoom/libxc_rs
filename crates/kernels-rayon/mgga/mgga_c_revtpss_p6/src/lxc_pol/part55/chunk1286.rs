//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1286/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1286(t125507: f64, t128303: f64, t128305: f64, t128306: f64, t128317: f64, t128319: f64, t128321: f64, t128324: f64, t128326: f64, t27123: f64, t28709: f64, t33311: f64, t4248: f64, t651: f64, t7373: f64, t8233: f64, t8764: f64, t8892: f64) -> f64 {
    let t130975 = -2.0_f64 * t651 * t7373 * t8233 - 2.0_f64 * t27123 * t8892 - t28709 * t8764 - 2.0_f64 * t33311 * t4248 - t125507 - 2.0_f64 * t128303 - 2.0_f64 * t128305 - 2.0_f64 * t128306 - t128317 - t128319 - t128321 - t128324 + t128326;
    t130975
}
