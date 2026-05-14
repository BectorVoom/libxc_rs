//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 960/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk960<F: Float>(t301: F, t4256: F, t7450: F, t9536: F, t2030: F, t372: F, t4262: F, t9563: F, t30265: F, t30273: F, t36925: F, t38976: F, t38978: F, t38982: F, t38986: F, t38990: F, t38994: F, t38996: F, t39000: F, t39002: F, t39005: F, t39009: F) -> (F,) {
    let t39013 = t7450 * t4256 * t9536 * t301;
    let t39017 = t2030 * t4262 * t9536 * t372;
    let t39021 = t2030 * t4262 * t9563 * t301;
    let t39024 = -0.31448092289604152068e-2 * t38976 + 0.18868855373762491241e-2 * t38978 - 0.37737710747524982482e-2 * t38982 - 0.20965394859736101378e-3 * t30265 - t36925 + t38986 / 96.0 - 0.42874018118069736972e-3 * t38990 + 0.42874018118069736972e-3 * t38994 + 0.68598428988911579156e-2 * t38996 - 0.7862023072401038017e-3 * t39000 - 11.0 / 96.0 * t39002 - 0.4584375e-1 * t39005 - 0.4584375e-1 * t39009 - 0.4584375e-1 * t39013 - 0.22921875e-1 * t39017 - 0.22921875e-1 * t39021 + 0.10718504529517434243e-3 * t30273;
    (t39024,)
}
