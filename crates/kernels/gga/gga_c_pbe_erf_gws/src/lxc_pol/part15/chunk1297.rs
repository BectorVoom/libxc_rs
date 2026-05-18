//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1297/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1297<F: Float>(t6645: F, t8991: F, t51351: F, t9612: F, t51350: F, t6684: F, t9641: F, t3249: F, t6238: F, t899: F, t923: F, t2209: F, t4026: F, t863: F) -> (F, F, F, F, F) {
    let t54043 = t6645 * t8991;
    let t54045 = t51351 * t9612;
    let t54047 = t6684 * t51350;
    let t54048 = t54047 * t9641;
    let t54052 = t899 * t6238 * t923 * t3249;
    let t54053 = F::new(7.0) / F::new(192.0) * t54052;
    let t54055 = t863 * t4026 * t2209;
    (t54043, t54045, t54048, t54053, t54055)
}
