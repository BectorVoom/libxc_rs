//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1075/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1075<F: Float>(t51200: F, t907: F, t13806: F, t915: F, t2276: F, t1477: F, t345: F, t56: F, t859: F, t854: F, t932: F, t2118: F, t2132: F, t2263: F, t331: F, t863: F) -> (F, F, F, F, F, F, F, F) {
    let t51201 = t51200 * t907;
    let t51213 = t13806 * t915;
    let t51214 = t2276 * t51213;
    let t51221 = t345 * t1477 * t56 * t859;
    let t51222 = t854 * t51221;
    let t51255 = t2276 * t13806 * t932;
    let t51266 = t2118 * t2132;
    let t51274 = t863 * t2263 * t331 * t56;
    (t51201, t51213, t51214, t51221, t51222, t51255, t51266, t51274)
}
