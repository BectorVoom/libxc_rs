//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 334/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk334<F: Float>(t1693: F, t45: F, t55: F, t1692: F, t1690: F, t12: F, t51: F) -> (F, F, F) {
    let t1696 = F::new(1.0) / t45 / t1693 / t55;
    let t1697 = t1692 * t1696;
    let t1698 = t1690 * t1697;
    let t1701 = t51 * t12;
    (t1696, t1698, t1701)
}
