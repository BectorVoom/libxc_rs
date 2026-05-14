//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 511/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk511<F: Float>(t5537: F, t7837: F, t51: F, t5566: F, t1608: F, t35: F, t428: F, t5568: F, t5567: F, t5596: F, t409: F, t64: F) -> (F, F, F, F, F, F, F) {
    let t22597 = t7837 * t5537;
    let t22602 = t5566 * t51;
    let t22603 = t1608 * t22602;
    let t22604 = t35 * t428;
    let t22613 = t7837 * t5568;
    let t22619 = t1608 * t5596 * t5567;
    let t22623 = t64 * t409;
    (t22597, t22602, t22603, t22604, t22613, t22619, t22623)
}
