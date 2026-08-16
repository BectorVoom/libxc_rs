//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1299/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1299<F: Float>(t1135: F, t9246: F, t2134: F, t54043: F, t54045: F, t54048: F, t54053: F, t54057: F, t54059: F, t54061: F, t54063: F, t54065: F, t54067: F, t54069: F) -> F {
    let t54071 = t9246 * t1135;
    let t54072 = t2134 * t54071;
    let t54073 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54072;
    let t54074 = t54043 / F::cast_from(24.0_f64) + t54045 / F::cast_from(384.0_f64) + t54048 / F::cast_from(64.0_f64) - t54053 - t54057 / F::cast_from(8.0_f64) - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t54059 + t54061 / F::cast_from(96.0_f64) + t54063 / F::cast_from(384.0_f64) - t54065 / F::cast_from(192.0_f64) + t54067 / F::cast_from(192.0_f64) - t54069 / F::cast_from(32.0_f64) + t54073;
    t54074
}
