//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1830/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1830<F: Float>(t243: F, t7021: F, t2732: F, t64: F, t9731: F, t2710: F, t826: F, t10631: F, t10886: F, t7028: F, t159: F, t8779: F) -> (F, F, F, F, F) {
    let t92978 = t7021 * t243;
    let t92979 = t92978 * t2732;
    let t92986 = t64 * t9731;
    let t92988 = t2710 * t92986 * t826;
    let t92991 = t10886 * t7028 * t10631;
    let t92993 = t8779 * t159;
    (t92979, t92986, t92988, t92991, t92993)
}
