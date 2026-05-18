//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 912/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk912<F: Float>(t14108: F, t2568: F, t242: F, t1162: F, t2399: F, t89: F, t18: F, t505: F, t3885: F, t2606: F, t3892: F, t3891: F) -> (F, F, F, F, F, F) {
    let t14109 = t2568 * t14108;
    let t14110 = t242 * t14109;
    let t14114 = t89 * t2399 * t1162;
    let t14116 = t18 * t505;
    let t14117 = t3885 * t14116;
    let t14118 = t2606 * t14117;
    let t14121 = t3892 * t14116;
    let t14122 = t3891 * t14121;
    (t14109, t14110, t14114, t14116, t14118, t14122)
}
