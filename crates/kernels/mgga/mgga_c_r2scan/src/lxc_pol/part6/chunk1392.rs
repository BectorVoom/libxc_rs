//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1392/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1392<F: Float>(t1748: F, t7741: F, t2788: F, t5938: F, t5: F, t7007: F, t736: F, t1754: F, t21121: F, t21125: F, t21129: F, t21133: F, t21136: F, t21139: F, t21149: F, t21151: F, t26396: F) -> (F,) {
    let t26398 = t7741 * t1748;
    let t26399 = 0.21687162600603479684e-1 * t26398;
    let t26400 = t2788 * t5938;
    let t26403 = t7007 * t5 * t736;
    let t26405 = t7741 * t1754;
    let t26406 = 0.32530743900905219526e-1 * t26405;
    let t26408 = -0.48024514811839999998e-1 * t21121 - 0.12154685976e1 * t26396 + t26399 + 0.21687162600603479684e-1 * t26400 - 0.16265371950452609763e-1 * t26403 - t26406 - t21125 - t21129 - t21133 - t21136 - t21139 + t21149 - 0.93505639170679904297e3 * t21151;
    (t26408,)
}
