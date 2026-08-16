//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1083/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1083(t2030: f64, t301: f64, t4262: f64, t9563: f64, t30265: f64, t30273: f64, t36925: f64, t38976: f64, t38978: f64, t38982: f64, t38986: f64, t38990: f64, t38994: f64, t38996: f64, t39000: f64, t39002: f64, t39005: f64, t39009: f64, t39013: f64, t39017: f64) -> f64 {
    let t39021 = t2030 * t4262 * t9563 * t301;
    let t39024 = -0.31448092289604152068e-2_f64 * t38976 + 0.18868855373762491241e-2_f64 * t38978 - 0.37737710747524982482e-2_f64 * t38982 - 0.20965394859736101378e-3_f64 * t30265 - t36925 + t38986 / 96.0_f64 - 0.42874018118069736972e-3_f64 * t38990 + 0.42874018118069736972e-3_f64 * t38994 + 0.68598428988911579156e-2_f64 * t38996 - 0.7862023072401038017e-3_f64 * t39000 - 11.0_f64 / 96.0_f64 * t39002 - 0.4584375e-1_f64 * t39005 - 0.4584375e-1_f64 * t39009 - 0.4584375e-1_f64 * t39013 - 0.22921875e-1_f64 * t39017 - 0.22921875e-1_f64 * t39021 + 0.10718504529517434243e-3_f64 * t30273;
    t39024
}
