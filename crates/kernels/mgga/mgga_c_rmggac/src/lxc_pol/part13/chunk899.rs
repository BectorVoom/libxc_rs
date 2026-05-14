//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 899/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk899<F: Float>(t9056: F, t9071: F, t9073: F, t10386: F, t10487: F, t10496: F, t10503: F, t37186: F, t8204: F, t8207: F, t8208: F, t8211: F, t9088: F, t9621: F, t9625: F, t9628: F) -> (F, F, F, F, F, F) {
    let t42552 = 0.5107751987195740728e-4 * t9056;
    let t42554 = 0.11974241701863808564e0 * t9071;
    let t42555 = 0.11974241701863808564e0 * t9073;
    let t42557 = t10386 - t42554 - t42555 + t10487 + t8204 - t37186 - t8207 + 4.0 * t8208 - t10496 - t8211 + t10503;
    let t42559 = 0.1702583995731913576e-4 * t9088;
    let t42560 = 0.23948483403727617128e0 * t9621;
    let t42561 = 0.23948483403727617128e0 * t9625;
    let t42562 = 0.23948483403727617128e0 * t9628;
    (t42552, t42557, t42559, t42560, t42561, t42562)
}
