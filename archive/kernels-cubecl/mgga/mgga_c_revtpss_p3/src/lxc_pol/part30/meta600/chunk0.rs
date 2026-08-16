//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2061/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2061<F: Float>(t3650: F, t7623: F, t12881: F, t7624: F, t12854: F, t29096: F, t13089: F, t12886: F, t12948: F, t26849: F, t26852: F, t3636: F) -> (F, F, F, F, F, F, F) {
    let t97138 = t3650 * t7623;
    let t97141 = t7624 * t12881;
    let t97149 = t12854 * t29096;
    let t97154 = t7624 * t13089;
    let t97161 = t7624 * t12886;
    let t97169 = t26849 * t12948;
    let t97171 = t26852 * t3636;
    (t97138, t97141, t97149, t97154, t97161, t97169, t97171)
}
