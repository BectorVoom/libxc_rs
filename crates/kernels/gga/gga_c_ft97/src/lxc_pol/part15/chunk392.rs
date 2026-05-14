//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 392/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk392<F: Float>(t1766: F, t965: F, t1775: F, t959: F, t1780: F, t2: F, t463: F, t17: F, t3050: F, t9: F, t458: F, t963: F, t942: F, t2981: F, t3006: F, t376: F, t89: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3119 = t1766 * t965;
    let t3125 = t1775 * t959;
    let t3127 = t1780 * t2;
    let t3134 = t463 * t2;
    let t3139 = t9 * t3050 * t17;
    let t3144 = t458 * t963;
    let t3149 = t2 * t942;
    let t3161 = t2981 / 27.0;
    let t3166 = t3006 / 9.0;
    let t3177 = t89 * t376 * t973;
    (t3119, t3125, t3127, t3134, t3139, t3144, t3149, t3161, t3166, t3177)
}
