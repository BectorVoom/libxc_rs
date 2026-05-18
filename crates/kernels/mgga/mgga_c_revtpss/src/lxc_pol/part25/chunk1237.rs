//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1237/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1237<F: Float>(t10761: F, t93015: F, t92979: F, t92982: F, t92984: F, t92989: F, t92991: F, t92996: F, t92998: F, t93000: F, t93001: F, t93004: F, t93008: F, t93010: F, t93013: F) -> F {
    let t93016 = t93015 * t10761;
    let t93018 = -F::new(7.0) / F::new(16.0) * t92979 - t92982 / F::new(4.0) + F::new(3.0) / F::new(16.0) * t92984 - t92989 + F::new(0.60984003371142393869e-4) * t92991 - t92996 - t92998 + t93000 - F::new(0.18292914397043087774e-2) * t93001 + F::new(0.17149607247227894789e-3) * t93004 + t93008 - F::new(0.85748036236139473943e-3) * t93010 - t93013 - F::new(0.27107389498472794076e-4) * t93016;
    t93018
}
