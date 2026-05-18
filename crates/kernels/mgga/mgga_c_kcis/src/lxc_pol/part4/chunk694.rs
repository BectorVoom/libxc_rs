//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 694/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk694<F: Float>(t1372: F, t3970: F, t1368: F, t1381: F, t25: F, t493: F, t534: F) -> (F, F, F, F, F) {
    let t3971 = t3970 * t1372;
    let t3972 = t1368 * t3971;
    let t3974 = t25 * t1381;
    let t3975 = t493 * t3974;
    let t3977 = F::new(1.0) / t534;
    (t3971, t3972, t3974, t3975, t3977)
}
