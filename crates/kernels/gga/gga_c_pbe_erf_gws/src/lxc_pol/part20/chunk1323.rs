//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1323/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1323<F: Float>(t11511: F, t4023: F, t11878: F, t11883: F, t14028: F, t3749: F, t11924: F, t51334: F, t854: F, t14024: F, t3788: F, t11651: F, t338: F, t54090: F) -> (F, F, F, F, F, F, F) {
    let t56929 = t11511 * t4023;
    let t56931 = t11878 * t4023;
    let t56933 = t11883 * t4023;
    let t56935 = t14028 * t3749;
    let t56937 = t51334 * t11924;
    let t56938 = t854 * t56937;
    let t56940 = t3788 * t14024;
    let t56943 = t54090 * t338 * t11651;
    (t56929, t56931, t56933, t56935, t56938, t56940, t56943)
}
