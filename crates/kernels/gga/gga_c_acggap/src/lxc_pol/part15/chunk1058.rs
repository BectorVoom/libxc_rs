//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1058/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1058<F: Float>(t30265: F, t34068: F, t34076: F, t38976: F, t38978: F, t38982: F, t38986: F, t38990: F, t38994: F, t38996: F, t39000: F, t39002: F, t39005: F, t39009: F, t39013: F, t39017: F, t39021: F) -> (F,) {
    let t41441 = -0.62896184579208304137e-2 * t38976 + 0.37737710747524982482e-2 * t38978 - 0.75475421495049964966e-2 * t38982 - 0.41930789719472202758e-3 * t30265 - 0.17149607247227894789e-2 * t34068 + t38986 / 48.0 - 0.85748036236139473944e-3 * t38990 + 0.85748036236139473944e-3 * t38994 + 0.13719685797782315831e-1 * t38996 - 0.15724046144802076034e-2 * t39000 - 11.0 / 48.0 * t39002 - 0.916875e-1 * t39005 - 0.916875e-1 * t39009 - 0.916875e-1 * t39013 - 0.4584375e-1 * t39017 - 0.4584375e-1 * t39021 + t34076;
    (t41441,)
}
