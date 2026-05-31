//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1344/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1344<F: Float>(t338: F, t36144: F, t36158: F, t36173: F, t36188: F, t36204: F, t36218: F, t36233: F, t36248: F, t12153: F, t2822: F, t2469: F, t3449: F, t3622: F) -> (F, F, F) {
    let t36252 = (t36144 + t36158 + t36173 + t36188 + t36204 + t36218 + t36233 + t36248) * t338;
    let t36255 = t12153 * t2822;
    let t36259 = F::cast_from(4.0_f64) * t2469 * t3622 * t3449;
    (t36252, t36255, t36259)
}
