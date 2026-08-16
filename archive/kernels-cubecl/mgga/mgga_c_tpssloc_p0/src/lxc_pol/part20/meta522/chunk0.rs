//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2056/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2056<F: Float>(t12440: F, t225: F, t32253: F, t59: F, t154: F, t541: F, t12289: F, t1336: F, t835: F, t12293: F, t12364: F, t3777: F) -> (F, F, F, F, F, F) {
    let t39919 = t12440 * t225;
    let t39933 = t59 * t32253;
    let t39934 = t39933 * t154;
    let t39936 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t39934 * t541;
    let t39944 = t1336 * t12289 * t835;
    let t39945 = t39944 * t12293;
    let t39947 = t3777 * t12364;
    (t39919, t39933, t39934, t39936, t39945, t39947)
}
