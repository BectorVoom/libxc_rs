//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1160/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1160<F: Float>(t11915: F, t4049: F, t11981: F, t4028: F, t54103: F, t54114: F, t54118: F, t56929: F, t56931: F, t56933: F, t56935: F, t56938: F, t56940: F, t56943: F, t56945: F, t3123: F, t8897: F) -> (F, F) {
    let t56947 = t4049 * t11915;
    let t56949 = t4028 * t11981;
    let t56951 = t56929 / 96.0 + t56931 / 96.0 + t56933 / 96.0 - 7.0 / 1152.0 * t56935 + t56938 / 16.0 + t54103 - 7.0 / 288.0 * t56940 - t56943 / 12.0 + t54114 + t54118 - t56945 / 96.0 - 5.0 / 64.0 * t56947 - t56949 / 48.0;
    let t56954 = t3123 * t8897;
    (t56951, t56954)
}
