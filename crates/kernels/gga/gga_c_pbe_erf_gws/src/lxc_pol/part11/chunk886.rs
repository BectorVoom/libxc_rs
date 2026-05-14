//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 886/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk886<F: Float>(t142: F, t985: F, t10207: F, t751: F, t1472: F, t168: F, t3609: F, t1383: F, t3380: F, t11159: F, t700: F, t10033: F, t242: F, t1365: F, t153: F, t3373: F) -> (F, F, F, F, F, F, F) {
    let t34302 = t985 * t142;
    let t34326 = t751 * t10207;
    let t34334 = t168 * t1472 * t3609;
    let t34336 = t3380 * t1383;
    let t34340 = t11159 * t700;
    let t34360 = t10033 * t242;
    let t34371 = t153 * t1365 * t3373;
    (t34302, t34326, t34334, t34336, t34340, t34360, t34371)
}
