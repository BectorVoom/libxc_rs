//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 810/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk810<F: Float>(t218: F, t5108: F, t213: F, t198: F, t7776: F, t185: F, t1464: F, t671: F, t219: F, t5463: F, t16712: F, t197: F, t155: F, t1660: F, t5399: F, t56: F) -> (F, F, F, F, F, F, F) {
    let t16823 = 1.0 / t5108 / t218;
    let t16824 = t213 * t16823;
    let t16843 = t7776 * t198;
    let t16845 = 112.0 / 1215.0 * t185 * t16843;
    let t16876 = 0.44134814814814814812e-2 * t1464 * t671;
    let t16904 = t5463 * t219;
    let t16932 = t197 * t16712;
    let t16942 = t155 * t1660;
    let t16970 = t56 * t5399;
    (t16824, t16845, t16876, t16904, t16932, t16942, t16970)
}
