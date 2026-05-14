//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 841/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk841<F: Float>(t1553: F, t7236: F, t502: F, t7271: F, t505: F, t97: F, t120: F, t5772: F, t131: F, t137: F, t5852: F, t133: F, t4573: F, t1473: F, t1497: F, t5615: F, t751: F) -> (F, F, F, F, F, F, F, F) {
    let t19357 = 0.29018074074074074074e1 * t1553 * t7236;
    let t19359 = 0.25390814814814814815e1 * t502 * t7271;
    let t19367 = 1.0 / t505 / t97;
    let t19383 = t5772 * t120;
    let t19407 = t131 / t5852 / t137;
    let t19439 = 0.29801938271604938271e1 * t133 * t4573 * t120;
    let t19458 = 0.31931290694012290916e0 * t1473 * t1497;
    let t19466 = 0.79828226735030727292e-1 * t751 * t5615;
    (t19357, t19359, t19367, t19383, t19407, t19439, t19458, t19466)
}
