//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1266/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1266<F: Float>(t3291: F, t51214: F, t14007: F, t9485: F, t14015: F, t9540: F, t9619: F, t14063: F, t8962: F, t854: F, t14064: F, t3113: F) -> (F, F, F, F, F, F) {
    let t54014 = t51214 * t3291;
    let t54016 = t14007 * t9485;
    let t54019 = t14015 * t9540;
    let t54021 = t14015 * t9619;
    let t54023 = t14063 * t8962;
    let t54024 = t854 * t54023;
    let t54027 = t3113 * t14064;
    (t54014, t54016, t54019, t54021, t54024, t54027)
}
