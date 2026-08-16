//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1284/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1284(t104115: f64, t111734: f64, t124169: f64, t128240: f64, t128242: f64, t128244: f64, t128245: f64, t128251: f64, t128254: f64, t128256: f64, t128260: f64, t128261: f64, t128266: f64, t128270: f64, t128273: f64, t130928: f64, t130929: f64, t130932: f64, t130946: f64, t1518: f64, t2055: f64, t29427: f64, t32175: f64, t32177: f64, t33287: f64, t33645: f64, t4292: f64, t569: f64, t670: f64, t7367: f64, t8563: f64) -> f64 {
    let t130951 = -2.0_f64 * t29427 * t7367 + t128240 + t128242 - t128244 - t128245 - t128251 - t128254 - t128256 + t128260 - t128261 - t128266 + t128270 - t128273 + (2.0_f64 * t104115 * t2055 + 2.0_f64 * t111734 * t2055 + 2.0_f64 * t124169 * t1518 + 2.0_f64 * t130929 * t670 + 2.0_f64 * t130932 * t1518 + 2.0_f64 * t33287 * t4292 + t130928 + 2.0_f64 * t130946 + 2.0_f64 * t32175 + 2.0_f64 * t32177 + 2.0_f64 * t33645 + 2.0_f64 * t8563) * t569;
    t130951
}
