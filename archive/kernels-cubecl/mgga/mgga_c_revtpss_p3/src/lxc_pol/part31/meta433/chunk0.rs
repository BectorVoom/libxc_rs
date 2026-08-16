//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1550/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1550<F: Float>(t19501: F, t3095: F, t3092: F, t1043: F, t3155: F, t6271: F, t3117: F, t12131: F, t357: F, t4786: F, t6100: F, t1065: F, t6244: F) -> (F, F, F, F, F) {
    let t19625 = t19501 * t3095;
    let t19626 = t3092 * t19625;
    let t19634 = t3155 * t1043;
    let t19635 = t6271 * t19634;
    let t19636 = t3117 * t19635;
    let t19639 = t12131 * t357;
    let t19640 = t6271 * t19639;
    let t19641 = t3117 * t19640;
    let t19644 = t6100 * t4786;
    let t19645 = t3092 * t19644;
    let t19649 = t1065 * t6244;
    (t19626, t19636, t19641, t19645, t19649)
}
