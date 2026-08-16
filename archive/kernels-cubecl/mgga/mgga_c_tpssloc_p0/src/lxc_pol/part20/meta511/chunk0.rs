//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2027/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2027<F: Float>(t12458: F, t12461: F, t677: F, t9713: F, t3684: F, t181: F, t2558: F, t686: F, t1291: F, t2369: F, t9720: F, t9843: F) -> (F, F, F, F, F, F) {
    let t39350 = t12458 * t12461;
    let t39354 = t677 * t9713;
    let t39356 = F::cast_from(0.21687162600603479684e-1_f64) * t3684 * t39354;
    let t39358 = t686 * t2558 * t181;
    let t39360 = F::cast_from(0.18989649058080861537e-2_f64) * t1291 * t39358;
    let t39362 = t9720 * t2369 * t9843;
    (t39350, t39354, t39356, t39358, t39360, t39362)
}
