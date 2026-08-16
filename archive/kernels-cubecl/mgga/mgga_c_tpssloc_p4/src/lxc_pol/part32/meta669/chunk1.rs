//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2102/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2102<F: Float>(t27495: F, t27497: F, t95195: F, t1170: F, t2121: F, t27732: F, t15590: F, t7338: F, t27614: F, t3572: F, t27617: F, t3523: F) -> (F, F, F, F, F) {
    let t95201 = t95195 * t27495 * t27497;
    let t95213 = F::cast_from(0.54831135561607547884e-2_f64) * t2121 * t1170 * t27732;
    let t95238 = t15590 * t7338;
    let t95242 = t27614 * t3572 / F::cast_from(1152.0_f64);
    let t95244 = t27617 * t3523 / F::cast_from(1728.0_f64);
    (t95201, t95213, t95238, t95242, t95244)
}
