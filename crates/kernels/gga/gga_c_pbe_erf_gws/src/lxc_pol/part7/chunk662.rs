//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 662/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk662<F: Float>(t4933: F, t5121: F, t5186: F, t5319: F, t5382: F, t5439: F, t5499: F, t5566: F, t1472: F, t168: F, t738: F, t1931: F, t703: F, t1452: F, t153: F, t542: F) -> (F, F, F, F) {
    let t5569 = t4933 + t5121 + t5186 + t5319 + t5382 + t5439 + t5499 + t5566;
    let t5574 = t168 * t1472 * t738;
    let t5577 = t168 * t703 * t1931;
    let t5580 = t153 * t542 * t1452;
    (t5569, t5574, t5577, t5580)
}
