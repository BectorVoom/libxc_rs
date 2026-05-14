//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1103/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1103<F: Float>(t25981: F, t820: F, t843: F, t2681: F, t7262: F, t533: F, t816: F, t92993: F, t7259: F, t9709: F, t1389: F, t3964: F, t92986: F, t26009: F, t9802: F, t64: F, t9990: F) -> (F, F, F, F, F, F, F) {
    let t94455 = t820 * t25981 * t843;
    let t94459 = t820 * t7262 * t2681;
    let t94471 = t92993 * t533 * t816;
    let t94472 = 455.0 / 1296.0 * t94471;
    let t94473 = t7259 * t9709;
    let t94474 = 0.25692334753583138159e-2 * t94473;
    let t94476 = t3964 * t92986 * t1389;
    let t94477 = 0.16264433699083676445e-3 * t94476;
    let t94483 = t9802 * t26009;
    let t94484 = 0.91476005056713590805e-4 * t94483;
    let t94491 = t9990 * t64;
    (t94455, t94459, t94472, t94474, t94477, t94484, t94491)
}
