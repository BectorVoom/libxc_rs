//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 704/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk704<F: Float>(t10024: F, t801: F, t169: F, t301: F, t3373: F, t784: F, t3379: F, t532: F, t159: F, t285: F, t142: F, t3637: F) -> (F, F, F, F, F) {
    let t10025 = t10024 * t801;
    let t10029 = t169 * t784 * t3373 * t301;
    let t10033 = t532 * t3379;
    let t10035 = t10033 * t159 * t285;
    let t10037 = t142 * t3637;
    (t10025, t10029, t10033, t10035, t10037)
}
