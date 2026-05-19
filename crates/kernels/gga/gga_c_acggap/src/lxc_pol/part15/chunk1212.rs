//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1212/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1212<F: Float>(t30265: F, t34068: F, t34076: F, t38976: F, t38978: F, t38982: F, t38986: F, t38990: F, t38994: F, t38996: F, t39000: F, t39002: F, t39005: F, t39009: F, t39013: F, t39017: F, t39021: F) -> F {
    let t41441 = -F::cast_from(0.62896184579208304137e-2_f64) * t38976 + F::cast_from(0.37737710747524982482e-2_f64) * t38978 - F::cast_from(0.75475421495049964966e-2_f64) * t38982 - F::cast_from(0.41930789719472202758e-3_f64) * t30265 - F::cast_from(0.17149607247227894789e-2_f64) * t34068 + t38986 / F::new(48.0) - F::cast_from(0.85748036236139473944e-3_f64) * t38990 + F::cast_from(0.85748036236139473944e-3_f64) * t38994 + F::cast_from(0.13719685797782315831e-1_f64) * t38996 - F::cast_from(0.15724046144802076034e-2_f64) * t39000 - F::new(11.0) / F::new(48.0) * t39002 - F::new(0.916875e-1) * t39005 - F::new(0.916875e-1) * t39009 - F::new(0.916875e-1) * t39013 - F::new(0.4584375e-1) * t39017 - F::new(0.4584375e-1) * t39021 + t34076;
    t41441
}
