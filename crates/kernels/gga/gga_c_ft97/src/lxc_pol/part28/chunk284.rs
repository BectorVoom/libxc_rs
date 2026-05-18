//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 284/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk284<F: Float>(t1910: F, t3113: F, t1909: F, t1766: F, t965: F, t473: F, t91: F, t1775: F, t959: F, t1780: F, t2: F, t2984: F) -> (F, F, F, F, F) {
    let t3114 = t1910 * t3113;
    let t3115 = t1909 * t3114;
    let t3119 = t1766 * t965;
    let t3121 = t91 * t3119 * t473;
    let t3125 = t1775 * t959;
    let t3127 = t1780 * t2;
    let t3128 = t3127 * t2984;
    (t3114, t3115, t3121, t3125, t3128)
}
