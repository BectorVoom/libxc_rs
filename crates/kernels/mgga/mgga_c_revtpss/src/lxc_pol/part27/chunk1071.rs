//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1071/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1071<F: Float>(t93012: F, t2453: F, t2783: F, t64: F, t10761: F, t92979: F, t92982: F, t92984: F, t92989: F, t92991: F, t92996: F, t92998: F, t93000: F, t93001: F, t93004: F, t93008: F, t93010: F) -> (F,) {
    let t93013 = 0.22589491248727328397e-6 * t93012;
    let t93015 = t2453 * t2783 * t64;
    let t93016 = t93015 * t10761;
    let t93018 = -7.0 / 16.0 * t92979 - t92982 / 4.0 + 3.0 / 16.0 * t92984 - t92989 + 0.60984003371142393869e-4 * t92991 - t92996 - t92998 + t93000 - 0.18292914397043087774e-2 * t93001 + 0.17149607247227894789e-3 * t93004 + t93008 - 0.85748036236139473943e-3 * t93010 - t93013 - 0.27107389498472794076e-4 * t93016;
    (t93018,)
}
