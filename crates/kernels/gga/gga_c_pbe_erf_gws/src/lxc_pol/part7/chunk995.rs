//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 995/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk995<F: Float>(t17219: F, t17222: F, t17225: F, t17229: F, t17232: F, t17234: F, t17237: F, t17239: F, t17246: F, t17251: F, t17255: F, t17257: F, t17259: F, t17264: F, t17267: F, t17271: F, t17275: F, t17279: F, t17282: F, t17285: F, t17287: F, t17291: F, t17293: F) -> (F, F) {
    let t18228 = -t17219 + t17222 - t17225 + t17229 - t17232 + t17234 + t17237 + t17239 + t17246 + t17251 + t17255;
    let t18229 = -t17257 + t17259 - t17264 + t17267 - t17271 + t17275 - t17279 - t17282 + t17285 + t17287 + t17291 - t17293;
    (t18228, t18229)
}
