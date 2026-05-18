//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1318/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1318<F: Float>(t3123: F, t51338: F, t1125: F, t51292: F, t14101: F, t8921: F, t14024: F, t3120: F, t14011: F, t9626: F, t14547: F, t28024: F, t6523: F) -> (F, F, F, F, F, F) {
    let t54265 = t3123 * t51338;
    let t54267 = t1125 * t51292;
    let t54268 = F::new(7.0) / F::new(72.0) * t54267;
    let t54269 = t14101 * t8921;
    let t54271 = t3120 * t14024;
    let t54272 = F::new(7.0) / F::new(144.0) * t54271;
    let t54273 = t14011 * t9626;
    let t54276 = t14547 * t6523 * t28024;
    (t54265, t54268, t54269, t54272, t54273, t54276)
}
