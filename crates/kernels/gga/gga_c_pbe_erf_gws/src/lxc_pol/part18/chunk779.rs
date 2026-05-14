//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 779/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk779<F: Float>(t242: F, t8048: F, t2519: F, t700: F, t2523: F, t1383: F, t992: F, t168: F, t2831: F, t703: F, t1072: F, t1472: F, t142: F, t2873: F, t2893: F, t501: F) -> (F, F, F, F, F, F, F, F) {
    let t8050 = 0.16752564107100880375e0 * t8048 * t242;
    let t8051 = t2519 * t700;
    let t8057 = 0.16752564107100880375e0 * t2523 * t700;
    let t8058 = t992 * t1383;
    let t8064 = 0.39794582218349216586e-1 * t168 * t703 * t2831;
    let t8066 = t168 * t1472 * t1072;
    let t8108 = t142 * t2873;
    let t8122 = t501 * t2893;
    (t8050, t8051, t8057, t8058, t8064, t8066, t8108, t8122)
}
