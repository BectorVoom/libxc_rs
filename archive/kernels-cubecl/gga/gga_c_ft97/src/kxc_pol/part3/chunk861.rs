//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 861/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk861<F: Float>(t1053: F, t363: F, t2992: F, t12709: F, t2983: F, t12714: F, t379: F, t4733: F, t9144: F, t4714: F, t609: F, t574: F, t605: F) -> (F, F, F, F) {
    let t17375 = t1053 * t363;
    let t17376 = t2992 * t17375;
    let t17377 = t12709 * t17376;
    let t17380 = t2983 * t17375;
    let t17381 = t12714 * t17380;
    let t17384 = t4733 * t379;
    let t17385 = t9144 * t17384;
    let t17388 = t4714 * t609;
    let t17390 = t574 * t605 * t17388;
    (t17377, t17381, t17385, t17390)
}
