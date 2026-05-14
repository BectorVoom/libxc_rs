//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 976/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk976<F: Float>(t11351: F, t11406: F, t12147: F, t12197: F, t12260: F, t9861: F, t9893: F, t9946: F, t1105: F, t2053: F, t944: F, t1123: F, t274: F, t9607: F, t1172: F, t318: F) -> (F, F, F, F, F, F) {
    let t12263 = t9861 + t9893 + t9946 + t11351 + t11406 + t12147 + t12197 + t12260;
    let t12275 = t2053 * t1105;
    let t12276 = t12275 * t944;
    let t13252 = t1123 * t274;
    let t13544 = t9607 * t13252;
    let t13756 = t1172 * t318;
    (t12263, t12275, t12276, t13252, t13544, t13756)
}
