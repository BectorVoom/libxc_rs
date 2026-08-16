//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1253/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1253<F: Float>(t54087: F, t14099: F, t863: F, t885: F, t1125: F, t51221: F, t3179: F, t51291: F, t854: F, t3228: F, t51465: F, t3224: F) -> (F, F, F, F, F, F, F) {
    let t54088 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54087;
    let t54090 = t863 * t14099 * t885;
    let t54094 = t1125 * t51221;
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54103 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54102;
    let t54113 = t51465 * t3228;
    let t54114 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54113;
    let t54117 = t51465 * t3224;
    (t54088, t54090, t54094, t54101, t54103, t54114, t54117)
}
