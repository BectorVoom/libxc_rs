//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1256/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1256<F: Float>(t4026: F, t828: F, t3287: F, t51255: F, t3142: F, t51382: F, t1125: F, t51292: F, t14024: F, t3120: F, t21296: F, t367: F, t899: F) -> (F, F, F, F, F, F) {
    let t54253 = t4026 * t828;
    let t54257 = t51255 * t3287;
    let t54258 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54257;
    let t54259 = t51382 * t3142;
    let t54260 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54259;
    let t54267 = t1125 * t51292;
    let t54268 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54267;
    let t54271 = t3120 * t14024;
    let t54272 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54271;
    let t54279 = t899 * t21296 * t367;
    (t54253, t54258, t54260, t54268, t54272, t54279)
}
