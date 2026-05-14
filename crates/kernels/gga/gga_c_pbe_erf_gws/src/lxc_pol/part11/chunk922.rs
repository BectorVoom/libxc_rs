//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 922/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk922<F: Float>(t13009: F, t723: F, t242: F, t42251: F, t12323: F, t281: F, t285: F, t545: F, t12978: F, t5651: F, t1105: F, t39749: F, t13086: F, t831: F, t2370: F, t830: F) -> (F, F, F, F, F, F) {
    let t43029 = t13009 * t723;
    let t43153 = t42251 * t242;
    let t43168 = t281 * t12323 * t545 * t285;
    let t43183 = t5651 * t12978;
    let t43223 = t39749 * t1105;
    let t43288 = t831 * t13086;
    let t43290 = t2370 * t830 * t43288;
    (t43029, t43153, t43168, t43183, t43223, t43290)
}
