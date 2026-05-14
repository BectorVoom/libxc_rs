//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 245/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk245<F: Float>(t192: F, t824: F, t852: F, t462: F, t847: F, t849: F, t92: F, t845: F, t91: F, t790: F, t795: F, t827: F) -> (F, F, F, F, F) {
    let t854 = t192 * t852 * t824;
    let t856 = -t847 - t462 * t849 / 3.0 - t92 * t854;
    let t858 = t91 * t845 * t856;
    let t860 = t790 / 9.0;
    let t863 = t858 / 6.0 - t860 - t795 / 9.0 - t827 / 3.0;
    (t854, t856, t858, t860, t863)
}
