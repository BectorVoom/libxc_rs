//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1083/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1083<F: Float>(t2030: F, t301: F, t4262: F, t9563: F, t30265: F, t30273: F, t36925: F, t38976: F, t38978: F, t38982: F, t38986: F, t38990: F, t38994: F, t38996: F, t39000: F, t39002: F, t39005: F, t39009: F, t39013: F, t39017: F) -> F {
    let t39021 = t2030 * t4262 * t9563 * t301;
    let t39024 = -F::cast_from(0.31448092289604152068e-2_f64) * t38976 + F::cast_from(0.18868855373762491241e-2_f64) * t38978 - F::cast_from(0.37737710747524982482e-2_f64) * t38982 - F::cast_from(0.20965394859736101378e-3_f64) * t30265 - t36925 + t38986 / F::cast_from(96.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t38990 + F::cast_from(0.42874018118069736972e-3_f64) * t38994 + F::cast_from(0.68598428988911579156e-2_f64) * t38996 - F::cast_from(0.7862023072401038017e-3_f64) * t39000 - F::cast_from(11.0_f64) / F::cast_from(96.0_f64) * t39002 - F::cast_from(0.4584375e-1_f64) * t39005 - F::cast_from(0.4584375e-1_f64) * t39009 - F::cast_from(0.4584375e-1_f64) * t39013 - F::cast_from(0.22921875e-1_f64) * t39017 - F::cast_from(0.22921875e-1_f64) * t39021 + F::cast_from(0.10718504529517434243e-3_f64) * t30273;
    t39024
}
