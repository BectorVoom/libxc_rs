//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1259/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1259<F: Float>(t3195: F, t4033: F, t4171: F, t51407: F, t14046: F, t3172: F, t14565: F, t346: F, t838: F, t859: F, t4142: F, t51529: F) -> (F, F, F, F, F) {
    let t54377 = t4033 * t3195;
    let t54378 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54377;
    let t54381 = t51407 * t4171;
    let t54397 = t14046 * t3172;
    let t54398 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54397;
    let t54401 = t14565 * t346 * t838 * t859;
    let t54402 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54401;
    let t54427 = t51529 * t4142;
    (t54378, t54381, t54398, t54402, t54427)
}
