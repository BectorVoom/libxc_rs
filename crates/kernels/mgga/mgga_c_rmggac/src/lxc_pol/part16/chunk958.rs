//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 958/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk958<F: Float>(t35691: F, t35705: F, t37815: F, t37816: F, t37818: F, t40343: F, t43390: F, t43391: F, t43392: F, t43393: F, t43422: F, t46992: F, t46995: F, t46999: F, t47004: F, t47006: F, t47008: F, t47011: F) -> (F,) {
    let t48849 = -0.1702583995731913576e-4 * t46992 + 0.20496175532535769483e-3 * t35691 - 0.1702583995731913576e-4 * t46995 + 0.59620292925746722033e-2 * t40343 - 0.19863479950205658386e-4 * t46999 + t43390 - t43391 + t43392 + t43393 - 0.39726959900411316773e-4 * t47004 + 0.19863479950205658386e-4 * t47006 - 0.79828278012425390427e-1 * t47008 - t37815 - t37816 - t37818 - 0.70441376091769752081e-2 * t35705 + 0.1702583995731913576e-4 * t47011 - t43422;
    (t48849,)
}
