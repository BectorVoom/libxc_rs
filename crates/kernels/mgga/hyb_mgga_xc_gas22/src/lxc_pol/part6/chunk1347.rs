//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1347/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1347<F: Float>(t24699: F, t3321: F, t24709: F, t3357: F, t8739: F, t8923: F, t8618: F, t8906: F, t10638: F, t6574: F, t10641: F, t6497: F) -> (F, F, F, F, F, F) {
    let t29398 = F::new(8.0) * t24699 * t3321;
    let t29400 = F::new(0.64327917994770140268e2) * t24709 * t3357;
    let t29402 = F::new(8.0) * t8923 * t8739;
    let t29404 = F::new(0.64327917994770140268e2) * t8906 * t8618;
    let t29406 = F::new(12.0) * t6574 * t10638;
    let t29408 = F::new(8.0) * t6497 * t10641;
    (t29398, t29400, t29402, t29404, t29406, t29408)
}
