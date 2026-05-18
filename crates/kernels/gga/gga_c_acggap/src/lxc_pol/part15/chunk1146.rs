//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1146/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1146<F: Float>(t15386: F, t31195: F, t39827: F, t2001: F, t6076: F, t1998: F, t6081: F, t1856: F, t7614: F, t6220: F, t1967: F, t9560: F) -> (F, F, F, F, F, F) {
    let t39829 = t31195 * t15386 * t39827;
    let t39831 = t2001 * t6076;
    let t39833 = t1998 * t6081;
    let t39835 = t7614 * t1856;
    let t39840 = t2001 * t6220;
    let t39842 = t1967 * t9560;
    (t39829, t39831, t39833, t39835, t39840, t39842)
}
