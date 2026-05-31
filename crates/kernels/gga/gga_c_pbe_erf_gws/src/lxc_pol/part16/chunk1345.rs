//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1345/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1345<F: Float>(t54397: F, t54401: F, t51461: F, t51466: F, t51473: F, t51479: F, t52715: F, t54391: F, t54394: F, t54404: F, t54406: F, t54408: F, t54411: F) -> F {
    let t55633 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54397;
    let t55634 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54401;
    let t55640 = -t52715 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t51461 - t54391 / F::cast_from(2.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51466 - t54394 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51473 + t55633 - t55634 - t54404 / F::cast_from(48.0_f64) - t54406 / F::cast_from(192.0_f64) - t54408 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t51479 - t54411 / F::cast_from(48.0_f64);
    t55640
}
