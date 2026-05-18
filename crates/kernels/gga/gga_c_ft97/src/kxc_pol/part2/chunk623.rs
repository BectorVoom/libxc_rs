//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 623/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk623<F: Float>(t360: F, t23: F, t7241: F, t174: F, t358: F, t1556: F, t357: F, t1589: F, t375: F, t89: F, t1636: F, t355: F) -> (F, F, F, F, F, F, F) {
    let t7741 = t360 * t360;
    let t7742 = F::new(1.0) / t7741;
    let t7750 = t23 * t7241;
    let t7760 = F::new(1.0) / t174 / t358;
    let t7763 = F::new(1.0) / t1556 / t357;
    let t7771 = t89 * t375 * t1589;
    let t7773 = t1636 * t355;
    (t7741, t7742, t7750, t7760, t7763, t7771, t7773)
}
