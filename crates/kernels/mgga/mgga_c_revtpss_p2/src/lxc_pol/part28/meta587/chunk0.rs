//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2054/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2054<F: Float>(t26009: F, t9802: F, t26004: F, t3961: F, t64: F, t9990: F, t2482: F, t596: F, t7262: F, t4021: F, t25986: F, t2661: F, t9980: F) -> (F, F, F, F, F, F) {
    let t94483 = t9802 * t26009;
    let t94484 = F::cast_from(0.91476005056713590805e-4_f64) * t94483;
    let t94485 = t26004 * t3961;
    let t94491 = t9990 * t64;
    let t94497 = t2482 * t7262 * t596;
    let t94498 = t94497 * t4021;
    let t94501 = t2661 * t25986 * t9980;
    (t94484, t94485, t94491, t94497, t94498, t94501)
}
