//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 768/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk768<F: Float>(t10402: F, t666: F, t89: F, t835: F, t9592: F, t446: F, t2404: F, t798: F, t2405: F, t824: F, t295: F, t9577: F) -> (F, F, F, F, F, F, F, F) {
    let t10404 = t89 * t666 * t10402;
    let t10406 = t835 * t9592;
    let t10407 = t446 * t10406;
    let t10409 = t2404 * t798;
    let t10410 = t2405 * t824;
    let t10411 = t10409 * t10410;
    let t10412 = t446 * t10411;
    let t10414 = t295 * t9577;
    (t10404, t10406, t10407, t10409, t10410, t10411, t10412, t10414)
}
