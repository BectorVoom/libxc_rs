//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 816/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk816<F: Float>(t1332: F, t147: F, t164: F, t1964: F, t762: F, t1464: F, t547: F, t528: F, t5975: F, t1371: F, t1480: F, t6046: F, t16465: F, t159: F, t285: F, t169: F, t2331: F, t274: F, t301: F) -> (F, F, F, F, F, F, F, F) {
    let t18075 = t1332 * t147;
    let t18077 = 0.14238371845981685628e-2 * t18075 * t164;
    let t18079 = 0.37806488667769341401e0 * t762 * t1964;
    let t18089 = 0.75612977335538682803e0 * t1464 * t547;
    let t18091 = 0.12602162889256447134e0 * t528 * t5975;
    let t18106 = 0.65586876954174354395e-3 * t6046 * t1371 * t1480;
    let t18108 = 0.12955432484775181115e-2 * t16465 * t1480;
    let t18122 = 0.3831185177913978998e-1 * t18075 * t159 * t285;
    let t18126 = 0.52404510650723236824e1 * t169 * t2331 * t274 * t301;
    (t18077, t18079, t18089, t18091, t18106, t18108, t18122, t18126)
}
