//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1675/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1675<F: Float>(t12365: F, t1354: F, t120: F, t3791: F, t1307: F, t3792: F, t3805: F, t1328: F, t210: F, t3719: F, t12178: F, t1343: F, t820: F) -> (F, F, F, F, F, F) {
    let t12366 = t12365 * t1354;
    let t12368 = t120 * t3791;
    let t12369 = t3792 * t1307;
    let t12371 = t3805 * t12368 * t12369;
    let t12375 = t210 * t1328 * t3719;
    let t12379 = t1343 * t820 * t12178;
    (t12366, t12368, t12369, t12371, t12375, t12379)
}
