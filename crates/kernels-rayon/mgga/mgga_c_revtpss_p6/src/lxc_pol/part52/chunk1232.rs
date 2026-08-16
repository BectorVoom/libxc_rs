//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1232/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1232(t32626: f64, t7937: f64, t119578: f64, t128235: f64, t128236: f64, t128240: f64, t128242: f64, t128244: f64, t128245: f64, t128251: f64, t128254: f64, t128256: f64, t128260: f64, t28718: f64, t28932: f64, t33913: f64, t7489: f64, t8568: f64) -> f64 {
    let t128261 = t32626 * t7937;
    let t128262 = -3.0_f64 * t119578 * t28718 + 3.0_f64 * t28932 * t8568 + 3.0_f64 * t33913 * t7489 - t128235 - t128236 + t128240 + t128242 - t128244 - t128245 - t128251 - t128254 - t128256 + t128260 - t128261;
    t128262
}
