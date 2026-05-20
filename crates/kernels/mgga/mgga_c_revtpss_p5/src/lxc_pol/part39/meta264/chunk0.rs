//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 985/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk985<F: Float>(t1317: F, t3853: F, t1320: F, t4029: F, t3855: F, t1333: F, t3863: F, t27: F, t583: F, t521: F, t19: F, t596: F) -> (F, F, F, F, F, F) {
    let t9395 = t1317 * t3853;
    let t9398 = t1320 * t4029;
    let t9404 = t1317 * t3855;
    let t9408 = t3863 * t1333;
    let t9410 = t583 * t27;
    let t9411 = t9410 * t521;
    let t9413 = t19 * t596;
    (t9395, t9398, t9404, t9408, t9411, t9413)
}
