//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1018/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1018<F: Float>(t2074: F, t875: F, t2407: F, t858: F, t6672: F, t2337: F, t904: F, t2189: F, t3140: F, t824: F, t2387: F, t6677: F, t6680: F, t6671: F, t6674: F, t2158: F, t814: F) -> (F, F, F, F, F, F, F, F) {
    let t20495 = t875 * t2074;
    let t20497 = t2407 * t858 * t20495;
    let t20499 = t6672 * t20497 / 4.0;
    let t20500 = t904 * t2337;
    let t20504 = t3140 * t2189;
    let t20505 = t824 * t20504;
    let t20509 = t2387 * t6677;
    let t20511 = t20509 * t6680 / 12.0;
    let t20512 = t2387 * t6671;
    let t20514 = t20512 * t6674 / 4.0;
    let t20515 = t2158 * t814;
    (t20495, t20499, t20500, t20504, t20505, t20511, t20514, t20515)
}
