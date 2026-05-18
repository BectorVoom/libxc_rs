//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 802/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk802<F: Float>(t12609: F, t9144: F, t2190: F, t3578: F, t574: F, t1026: F, t8232: F, t1882: F, t3463: F, t3590: F, t379: F, t569: F) -> (F, F, F, F, F) {
    let t12610 = t9144 * t12609;
    let t12614 = t574 * t3578 * t2190;
    let t12617 = t8232 * t1026;
    let t12620 = F::new(2.0) / F::new(27.0) * t1882 * t3463;
    let t12622 = t569 * t3590 * t379;
    (t12610, t12614, t12617, t12620, t12622)
}
