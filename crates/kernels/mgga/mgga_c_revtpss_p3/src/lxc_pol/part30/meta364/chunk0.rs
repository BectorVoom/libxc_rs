//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1390/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1390<F: Float>(t3618: F, t828: F, t1260: F, t3650: F, t3588: F, t73: F, t1209: F, t3781: F, t5330: F, t3153: F, t3601: F, t1284: F, t3555: F) -> (F, F, F, F, F, F, F) {
    let t12787 = t828 * t3618;
    let t12800 = t3650 * t1260;
    let t12803 = t3588 * t73;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12810 = t3601 * t3153;
    let t12831 = t3555 * t1284;
    (t12787, t12800, t12803, t12808, t12809, t12810, t12831)
}
