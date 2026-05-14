//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1136/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1136<F: Float>(t14015: F, t9470: F, t14007: F, t9366: F, t14093: F, t8848: F, t1135: F, t9246: F, t2134: F, t54043: F, t54045: F, t54048: F, t54053: F, t54057: F, t54059: F, t54061: F, t54063: F) -> (F,) {
    let t54065 = t14015 * t9470;
    let t54067 = t14007 * t9366;
    let t54069 = t8848 * t14093;
    let t54071 = t9246 * t1135;
    let t54072 = t2134 * t54071;
    let t54073 = 7.0 / 144.0 * t54072;
    let t54074 = t54043 / 24.0 + t54045 / 384.0 + t54048 / 64.0 - t54053 - t54057 / 8.0 - 5.0 / 192.0 * t54059 + t54061 / 96.0 + t54063 / 384.0 - t54065 / 192.0 + t54067 / 192.0 - t54069 / 32.0 + t54073;
    (t54074,)
}
