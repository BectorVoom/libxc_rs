//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1085/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1085<F: Float>(t94473: F, t1389: F, t3964: F, t92986: F, t26009: F, t9802: F, t64: F, t9990: F, t239: F, t820: F, t2482: F, t596: F, t7262: F, t25981: F, t27: F, t550: F, t7021: F) -> (F, F, F, F, F, F, F) {
    let t94474 = 0.25692334753583138159e-2 * t94473;
    let t94476 = t3964 * t92986 * t1389;
    let t94477 = 0.16264433699083676445e-3 * t94476;
    let t94483 = t9802 * t26009;
    let t94484 = 0.91476005056713590805e-4 * t94483;
    let t94491 = t9990 * t64;
    let t94493 = t820 * t94491 * t239;
    let t94497 = t2482 * t7262 * t596;
    let t94508 = t2482 * t25981 * t27;
    let t94513 = t7021 * t550;
    (t94474, t94477, t94484, t94493, t94497, t94508, t94513)
}
