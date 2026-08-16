//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 839/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk839<F: Float>(t1383: F, t992: F, t168: F, t2831: F, t703: F, t1072: F, t1472: F, t142: F, t2873: F, t2893: F, t501: F, t156: F, t4: F, t481: F) -> (F, F, F, F, F, F) {
    let t8058 = t992 * t1383;
    let t8064 = F::cast_from(0.39794582218349216586e-1_f64) * t168 * t703 * t2831;
    let t8066 = t168 * t1472 * t1072;
    let t8108 = t142 * t2873;
    let t8122 = t501 * t2893;
    let t8124 = t4 * t156 * t481;
    (t8058, t8064, t8066, t8108, t8122, t8124)
}
