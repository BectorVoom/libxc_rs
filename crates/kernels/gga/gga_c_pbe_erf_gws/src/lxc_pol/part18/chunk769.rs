//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 769/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk769<F: Float>(t5174: F, t1617: F, t732: F, t1672: F, t611: F, t185: F, t108: F, t615: F, t267: F) -> (F, F, F, F) {
    let t5175 = F::new(1.0) / t5174;
    let t5205 = t732 * t1617;
    let t5207 = t1672 * t611;
    let t5208 = t185 * t5207;
    let t5210 = t615 * t108;
    let t5211 = t5210 * t267;
    (t5175, t5205, t5208, t5211)
}
