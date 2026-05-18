//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 435/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk435<F: Float>(t3108: F, t348: F, t492: F, t920: F, t1910: F, t1909: F, t1766: F, t965: F, t473: F, t91: F, t1775: F, t959: F) -> (F, F, F, F, F, F) {
    let t3109 = t348 * t3108;
    let t3113 = t920 * t492;
    let t3114 = t1910 * t3113;
    let t3115 = t1909 * t3114;
    let t3119 = t1766 * t965;
    let t3121 = t91 * t3119 * t473;
    let t3125 = t1775 * t959;
    (t3109, t3114, t3115, t3119, t3121, t3125)
}
