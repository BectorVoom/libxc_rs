//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 521/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk521<F: Float>(t18: F, t464: F, t463: F, t458: F, t963: F, t1787: F, t3009: F, t2: F, t942: F, t1587: F, t432: F, t24: F, t3103: F, t469: F) -> (F, F, F, F, F, F, F) {
    let t3140 = t464 * t18;
    let t3141 = t463 * t3140;
    let t3144 = t458 * t963;
    let t3146 = t1787 * t3009;
    let t3149 = t2 * t942;
    let t3151 = t1587 * t3149 * t432;
    let t3155 = t24 * t469 * t3103;
    (t3140, t3141, t3144, t3146, t3149, t3151, t3155)
}
