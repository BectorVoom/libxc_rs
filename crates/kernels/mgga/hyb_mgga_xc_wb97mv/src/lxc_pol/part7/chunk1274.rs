//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1274/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1274<F: Float>(t26341: F, t3338: F, t9127: F, t9194: F, t9197: F, t26344: F, t9200: F, t26445: F, t3374: F, t9120: F, t9204: F, t9207: F, t26450: F, t9211: F, t10924: F, t6914: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31226 = 8.0 * t26341 * t3338;
    let t31228 = 8.0 * t9127 * t9194;
    let t31230 = 4.0 * t9127 * t9197;
    let t31232 = 0.19298375398431042081e3 * t26344 * t9200;
    let t31234 = 0.64327917994770140268e2 * t26445 * t3374;
    let t31236 = 0.64327917994770140268e2 * t9120 * t9204;
    let t31238 = 0.32163958997385070134e2 * t9120 * t9207;
    let t31240 = 0.1034520258385468006e4 * t26450 * t9211;
    let t31242 = 12.0 * t6914 * t10924;
    (t31226, t31228, t31230, t31232, t31234, t31236, t31238, t31240, t31242)
}
