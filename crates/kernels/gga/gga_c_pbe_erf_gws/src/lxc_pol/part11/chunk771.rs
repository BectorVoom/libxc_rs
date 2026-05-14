//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 771/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk771<F: Float>(t13242: F, t2147: F, t3116: F, t11787: F, t9035: F, t3763: F, t3781: F, t2255: F, t1123: F, t274: F) -> (F, F, F, F, F) {
    let t13243 = t2147 * t13242;
    let t13245 = t3116 * t13243 / 16.0;
    let t13247 = t9035 * t11787 / 16.0;
    let t13248 = t3781 * t3763;
    let t13249 = t2255 * t13248;
    let t13252 = t1123 * t274;
    (t13243, t13245, t13247, t13249, t13252)
}
