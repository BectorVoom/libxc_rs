//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1032/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1032<F: Float>(t12323: F, t169: F, t242: F, t299: F, t12324: F, t700: F, t100: F, t3644: F, t12891: F, t12381: F, t153: F, t542: F) -> (F, F, F, F, F) {
    let t42880 = t169 * t299 * t12323 * t242;
    let t42891 = t169 * t12324 * t700;
    let t42905 = t3644 * t100;
    let t42923 = t12891 * t700;
    let t42928 = t153 * t542 * t12381;
    (t42880, t42891, t42905, t42923, t42928)
}
