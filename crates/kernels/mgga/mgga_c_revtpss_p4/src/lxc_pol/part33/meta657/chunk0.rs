//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2112/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2112<F: Float>(t1468: F, t4343: F, t5962: F, t605: F, t6075: F, t775: F, t25207: F, t1583: F, t580: F, t98631: F, t27382: F, t29694: F, t689: F) -> (F, F, F, F, F, F) {
    let t105909 = t1468 * t4343;
    let t105919 = t605 * t5962;
    let t105923 = t6075 * t775;
    let t105924 = t25207 * t105923;
    let t105928 = t98631 * t580 * t1583;
    let t105930 = F::new(2.0) * t27382 * t105928;
    let t105933 = t29694 * t689;
    (t105909, t105919, t105923, t105924, t105930, t105933)
}
