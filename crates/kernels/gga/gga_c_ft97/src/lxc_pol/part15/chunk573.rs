//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 573/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk573<F: Float>(t327: F, t9577: F, t230: F, t2938: F, t9556: F, t2937: F, t325: F, t2440: F, t70: F, t9570: F, t1851: F, t971: F, t7773: F, t89: F, t921: F, t1636: F, t943: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10850 = t327 * t9577;
    let t10864 = t230 * t2938;
    let t10883 = 0.44934037037037037036e0 * t9556;
    let t10904 = 1.0 / t2937 / t325;
    let t10915 = t70 * t2440;
    let t10916 = t327 * t9570;
    let t10969 = t971 * t1851;
    let t11043 = t89 * t7773 * t921;
    let t11076 = t89 * t1636 * t943;
    (t10850, t10864, t10883, t10904, t10915, t10916, t10969, t11043, t11076)
}
