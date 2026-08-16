//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 777/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk777<F: Float>(t39: F, t535: F, t159: F, t285: F, t169: F, t301: F, t366: F, t745: F, t1354: F, t532: F, t1500: F, t2036: F) -> (F, F, F, F) {
    let t5668 = t39 * t535;
    let t5670 = t5668 * t159 * t285;
    let t5674 = t169 * t366 * t745 * t301;
    let t5676 = t532 * t1354;
    let t5678 = t5676 * t159 * t285;
    let t5680 = t1500 * t2036;
    (t5670, t5674, t5678, t5680)
}
