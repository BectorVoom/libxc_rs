//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 835/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk835<F: Float>(t11408: F, t2268: F, t6320: F, t6509: F, t13265: F, t484: F, t13296: F, t599: F, t475: F, t3516: F, t874: F) -> (F, F, F, F, F) {
    let t44375 = F::cast_from(0.17073003981405689759e0_f64) * t2268 * t6320 * t11408 * t6509;
    let t44376 = t484 * t13265;
    let t44377 = F::cast_from(0.47425011059460249332e-2_f64) * t44376;
    let t44381 = t599 * t13296;
    let t44382 = t44381 * t475;
    let t44386 = t3516 * t874;
    (t44375, t44377, t44381, t44382, t44386)
}
