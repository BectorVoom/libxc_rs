//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 927/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk927<F: Float>(t118: F, t119: F, t120: F, t1477: F, t1553: F, t7236: F, t502: F, t7271: F, t505: F, t97: F, t5772: F, t131: F, t137: F, t5852: F) -> (F, F, F, F, F, F) {
    let t19355 = F::new(70.0) / F::new(81.0) * t118 * t119 * t1477 * t120;
    let t19357 = F::new(0.29018074074074074074e1) * t1553 * t7236;
    let t19359 = F::new(0.25390814814814814815e1) * t502 * t7271;
    let t19367 = F::new(1.0) / t505 / t97;
    let t19383 = t5772 * t120;
    let t19407 = t131 / t5852 / t137;
    (t19355, t19357, t19359, t19367, t19383, t19407)
}
