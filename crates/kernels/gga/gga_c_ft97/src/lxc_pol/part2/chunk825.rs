//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 825/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk825<F: Float>(t3565: F, t558: F, t574: F, t605: F, t3541: F, t376: F, t89: F, t1882: F, t3452: F, t3457: F, t157: F, t1985: F) -> (F, F, F, F, F) {
    let t12956 = t3565 * t558;
    let t12958 = t574 * t605 * t12956;
    let t12963 = F::new(2.0) / F::new(9.0) * t89 * t376 * t3541;
    let t12965 = F::new(4.0) / F::new(9.0) * t1882 * t3452;
    let t12967 = F::new(2.0) / F::new(9.0) * t1882 * t3457;
    let t12968 = t1985 * t157;
    (t12958, t12963, t12965, t12967, t12968)
}
