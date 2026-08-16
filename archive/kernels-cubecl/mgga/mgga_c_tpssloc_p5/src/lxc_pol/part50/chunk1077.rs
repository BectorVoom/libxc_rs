//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1077/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1077<F: Float>(t31061: F, t31252: F, t3: F, t112: F, t8496: F, t1873: F, t23877: F, t23880: F, t7015: F, t6534: F, t7010: F, t12524: F, t8319: F) -> (F, F, F, F, F, F, F) {
    let t31253 = t31061 + t31252;
    let t31254 = t3 * t31253;
    let t31267 = t8496 * t112;
    let t31270 = t23877 * t1873;
    let t31272 = t23880 * t7015;
    let t31274 = t7010 * t6534;
    let t31277 = F::cast_from(27.0_f64) * t12524 * t8319;
    (t31253, t31254, t31267, t31270, t31272, t31274, t31277)
}
