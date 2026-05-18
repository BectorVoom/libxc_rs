//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 761/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk761<F: Float>(t2661: F, t89: F, t9725: F, t2724: F, t811: F, t2719: F, t816: F, t820: F, t272: F, t9606: F, t9525: F, t2697: F, t688: F) -> (F, F, F, F, F, F, F) {
    let t10286 = t89 * t9725 * t2661;
    let t10292 = t2724 * t811;
    let t10296 = t816 * t2719;
    let t10297 = t10296 * t820;
    let t10304 = F::new(1.0) / t272 / t9606;
    let t10305 = t10304 * t9525;
    let t10308 = t2697 * t688;
    (t10286, t10292, t10296, t10297, t10304, t10305, t10308)
}
