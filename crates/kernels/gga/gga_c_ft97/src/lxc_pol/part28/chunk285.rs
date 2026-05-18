//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 285/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk285<F: Float>(t1787: F, t2988: F, t2: F, t463: F, t2993: F, t17: F, t3050: F, t9: F, t18: F, t464: F, t458: F, t963: F) -> (F, F, F, F, F, F) {
    let t3131 = t1787 * t2988;
    let t3134 = t463 * t2;
    let t3135 = t3134 * t2993;
    let t3139 = t9 * t3050 * t17;
    let t3140 = t464 * t18;
    let t3141 = t463 * t3140;
    let t3144 = t458 * t963;
    (t3131, t3135, t3139, t3140, t3141, t3144)
}
