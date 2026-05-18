//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 332/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk332<F: Float>(t45: F, t55: F, t389: F, t44: F, t54: F, t52: F) -> (F, F, F) {
    let t1675 = F::new(1.0) / t45 / t55;
    let t1679 = t55 * t389;
    let t1681 = F::new(1.0) / t44 / t1679;
    let t1682 = t54 * t1681;
    let t1683 = t52 * t1682;
    (t1675, t1681, t1683)
}
