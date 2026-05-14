//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 651/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk651<F: Float>(t103: F, t2761: F, t332: F, t7875: F, t7877: F, t2627: F, t442: F, t919: F, t818: F, t1087: F, t2232: F, t1086: F, t1: F, t350: F, t786: F, t961: F) -> (F, F, F, F, F, F, F) {
    let t7880 = t2761 * t7875 * t332 * t7877 * t103;
    let t7920 = t2627 * t442;
    let t7921 = t919 * t7920;
    let t7927 = t442 * t818;
    let t7938 = t1087 * t2232;
    let t7939 = t1086 * t7938;
    let t7943 = t786 * t1 * t350;
    let t7944 = t961 * t7943;
    (t7880, t7921, t7927, t7938, t7939, t7943, t7944)
}
