//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 772/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk772<F: Float>(t432: F, t4551: F, t1852: F, t452: F, t3291: F, t942: F, t11863: F, t15959: F, t4431: F, t492: F, t1910: F, t1909: F) -> (F, F, F, F) {
    let t15994 = t4551 * t432;
    let t15996 = t452 * t1852 * t15994;
    let t16000 = t452 * t3291 * t942;
    let t16003 = t11863 * t15959;
    let t16006 = t4431 * t492;
    let t16007 = t1910 * t16006;
    let t16008 = t1909 * t16007;
    (t15996, t16000, t16003, t16008)
}
