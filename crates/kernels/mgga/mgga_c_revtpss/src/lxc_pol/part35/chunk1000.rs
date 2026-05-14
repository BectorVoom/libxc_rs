//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1000/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1000<F: Float>(t533: F, t816: F, t92993: F, t7259: F, t9709: F, t1389: F, t3964: F, t92986: F, t26009: F, t9802: F, t64: F, t9990: F, t239: F, t820: F, t2482: F, t596: F, t7262: F) -> (F, F, F, F, F, F) {
    let t94471 = t92993 * t533 * t816;
    let t94473 = t7259 * t9709;
    let t94476 = t3964 * t92986 * t1389;
    let t94483 = t9802 * t26009;
    let t94491 = t9990 * t64;
    let t94493 = t820 * t94491 * t239;
    let t94497 = t2482 * t7262 * t596;
    (t94471, t94473, t94476, t94483, t94493, t94497)
}
