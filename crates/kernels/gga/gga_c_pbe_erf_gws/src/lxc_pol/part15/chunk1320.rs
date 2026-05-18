//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1320/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1320<F: Float>(t51388: F, t51396: F, t4028: F, t9103: F, t14101: F, t8837: F, t9098: F, t14079: F, t3283: F, t4049: F, t9594: F, t1154: F, t51387: F) -> (F, F, F, F, F, F, F, F) {
    let t54293 = F::new(119.0) / F::new(1728.0) * t51388;
    let t54294 = F::new(119.0) / F::new(864.0) * t51396;
    let t54295 = t4028 * t9103;
    let t54297 = t14101 * t8837;
    let t54299 = t4028 * t9098;
    let t54301 = t14079 * t3283;
    let t54302 = F::new(7.0) / F::new(576.0) * t54301;
    let t54303 = t4049 * t9594;
    let t54305 = t51387 * t1154;
    (t54293, t54294, t54295, t54297, t54299, t54302, t54303, t54305)
}
