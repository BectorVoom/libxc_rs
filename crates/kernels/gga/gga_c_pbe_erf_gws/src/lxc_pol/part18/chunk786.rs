//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 786/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk786<F: Float>(t1378: F, t1971: F, t8361: F, t163: F, t169: F, t2979: F, t299: F, t2962: F, t679: F, t1049: F, t1986: F, t2007: F, t2970: F, t1: F, t2522: F, t3: F) -> (F, F, F, F, F, F) {
    let t8390 = t8361 * t1378 * t1971;
    let t8395 = 0.17961351015381913641e-1 * t169 * t299 * t2979 * t163;
    let t8404 = 8.0 / 3.0 * t2962 * t679;
    let t8405 = t1049 * t1986;
    let t8408 = t2970 * t2007;
    let t8411 = t2522 * t1 * t3;
    (t8390, t8395, t8404, t8405, t8408, t8411)
}
