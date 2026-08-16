//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1206/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1206<F: Float>(t3906: F, t898: F, t938: F, t3886: F, t6781: F, t12232: F, t810: F, t3721: F, t8734: F, t12098: F, t2376: F, t2494: F, t2501: F) -> (F, F, F, F, F, F, F) {
    let t35889 = t3906 * t898;
    let t35890 = t35889 * t938;
    let t35910 = t6781 * t3886;
    let t36000 = t12232 * t810;
    let t36007 = t8734 * t3721;
    let t36046 = t2376 * t12098;
    let t36089 = t2501 * t2494;
    (t35889, t35890, t35910, t36000, t36007, t36046, t36089)
}
