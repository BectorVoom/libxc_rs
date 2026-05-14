//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 987/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk987<F: Float>(t1938: F, t3892: F, t1907: F, t310: F, t464: F, t1219: F, t1937: F, t5384: F, t871: F, t6438: F, t857: F, t6558: F, t1220: F, t1914: F, t316: F, t879: F) -> (F, F, F, F, F, F) {
    let t19664 = t3892 * t1938;
    let t19667 = t310 * t1907;
    let t19668 = t19667 * t464;
    let t19672 = t5384 * t1219 * t1937 * t871;
    let t19676 = t857 * t6438;
    let t19678 = t857 * t6558;
    let t19688 = t316 * t1220 * t1914 * t879;
    (t19664, t19668, t19672, t19676, t19678, t19688)
}
