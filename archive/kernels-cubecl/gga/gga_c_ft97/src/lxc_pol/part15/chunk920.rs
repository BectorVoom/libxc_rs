//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 920/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk920<F: Float>(t21382: F, t6783: F, t2378: F, t37481: F, t21333: F, t30815: F, t4977: F, t1609: F, t694: F, t5005: F, t695: F, t1100: F, t52563: F) -> (F, F, F, F, F, F, F) {
    let t66419 = t6783 * t21382;
    let t66422 = t37481 * t2378;
    let t66424 = t30815 * t21333;
    let t66451 = t2378 * t4977;
    let t66482 = t694 * t1609;
    let t66520 = t695 * t5005;
    let t66555 = t1100 * t52563;
    (t66419, t66422, t66424, t66451, t66482, t66520, t66555)
}
