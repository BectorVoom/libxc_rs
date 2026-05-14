//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 397/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk397<F: Float>(t1910: F, t3113: F, t1909: F, t1766: F, t965: F, t473: F, t91: F, t1775: F, t959: F, t1780: F, t2: F, t2984: F, t1787: F, t2988: F, t463: F, t2993: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3114 = t1910 * t3113;
    let t3115 = t1909 * t3114;
    let t3119 = t1766 * t965;
    let t3121 = t91 * t3119 * t473;
    let t3125 = t1775 * t959;
    let t3127 = t1780 * t2;
    let t3128 = t3127 * t2984;
    let t3131 = t1787 * t2988;
    let t3134 = t463 * t2;
    let t3135 = t3134 * t2993;
    (t3114, t3115, t3119, t3121, t3125, t3127, t3128, t3131, t3134, t3135)
}
