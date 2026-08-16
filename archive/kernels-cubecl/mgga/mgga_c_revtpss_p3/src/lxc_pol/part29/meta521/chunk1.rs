//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1845/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1845<F: Float>(t64: F, t9990: F, t2482: F, t596: F, t7262: F, t4021: F, t25986: F, t2661: F, t9980: F, t26024: F, t3926: F, t4059: F) -> (F, F, F, F, F, F) {
    let t94491 = t9990 * t64;
    let t94497 = t2482 * t7262 * t596;
    let t94498 = t94497 * t4021;
    let t94501 = t2661 * t25986 * t9980;
    let t94503 = t26024 * t3926;
    let t94505 = t26024 * t4059;
    (t94491, t94497, t94498, t94501, t94503, t94505)
}
