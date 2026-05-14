//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 684/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk684<F: Float>(t15931: F, t15966: F, t348: F, t1882: F, t4603: F, t4599: F, t3291: F, t447: F, t925: F, t3052: F, t986: F, t379: F, t4623: F, t432: F, t4551: F, t1852: F, t452: F) -> (F, F, F, F, F, F, F) {
    let t15967 = t15931 + t15966;
    let t15968 = t348 * t15967;
    let t15978 = t1882 * t4603;
    let t15980 = t1882 * t4599;
    let t15983 = t447 * t3291 * t925;
    let t15987 = t447 * t986 * t3052;
    let t15991 = t447 * t4623 * t379;
    let t15994 = t4551 * t432;
    let t15996 = t452 * t1852 * t15994;
    (t15968, t15978, t15980, t15983, t15987, t15991, t15996)
}
