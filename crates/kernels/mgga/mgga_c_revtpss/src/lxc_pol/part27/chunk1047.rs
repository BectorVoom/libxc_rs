//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1047/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1047<F: Float>(t3618: F, t828: F, t3363: F, t5405: F, t12287: F, t5308: F, t12282: F, t5312: F, t1260: F, t3650: F, t3588: F, t73: F) -> (F, F, F, F, F) {
    let t12787 = t828 * t3618;
    let t12788 = t3363 * t5405;
    let t12789 = t12787 * t12788;
    let t12794 = t5308 * t12287;
    let t12797 = t5312 * t12282;
    let t12800 = t3650 * t1260;
    let t12803 = t3588 * t73;
    (t12789, t12794, t12797, t12800, t12803)
}
