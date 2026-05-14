//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 837/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk837<F: Float>(t24935: F, t25218: F, t25286: F, t25387: F, t2682: F, t317: F, t24989: F, t193: F, t824: F, t880: F, t6222: F, t2739: F, t24964: F, t6223: F, t6224: F, t681: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25389 = t24935 + t25218 + t25286 + t25387;
    let t25391 = t317 * t2682;
    let t25392 = t24989 * t25391;
    let t25393 = t193 * t25392;
    let t25395 = t880 * t824;
    let t25396 = t6222 * t25395;
    let t25397 = t193 * t25396;
    let t25400 = t317 * t2739;
    let t25401 = t6222 * t25400;
    let t25402 = t193 * t25401;
    let t25405 = t24964 * t6223;
    let t25406 = t193 * t25405;
    let t25409 = t681 * t6224;
    (t25389, t25391, t25392, t25393, t25395, t25396, t25397, t25400, t25401, t25402, t25405, t25406, t25409)
}
