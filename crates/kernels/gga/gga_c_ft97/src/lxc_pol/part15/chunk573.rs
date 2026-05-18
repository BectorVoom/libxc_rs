//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 573/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk573<F: Float>(t29: F, t32: F, t8991: F, t23: F, t7368: F, t143: F, t7763: F, t1642: F, t525: F, t7800: F, t10: F, t144: F, t3050: F) -> (F, F, F, F, F, F) {
    let t8994 = t8991 / t32 / t29;
    let t9016 = t23 * t7368;
    let t9025 = t143 * t7763;
    let t9049 = t1642 * t525;
    let t9054 = t143 * t7800;
    let t9071 = t10 * t3050 * t144;
    (t8994, t9016, t9025, t9049, t9054, t9071)
}
