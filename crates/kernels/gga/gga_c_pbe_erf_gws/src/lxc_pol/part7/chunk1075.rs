//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1075/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1075<F: Float>(t1509: F, t7236: F, t486: F, t7271: F, t118: F, t119: F, t120: F, t1477: F, t1553: F, t502: F, t102: F, t1563: F, t19268: F) -> (F, F, F, F, F, F) {
    let t19349 = F::cast_from(0.57738765432098765432e1_f64) * t1509 * t7236;
    let t19351 = F::cast_from(0.50521419753086419753e1_f64) * t486 * t7271;
    let t19355 = F::new(70.0) / F::new(81.0) * t118 * t119 * t1477 * t120;
    let t19357 = F::cast_from(0.29018074074074074074e1_f64) * t1553 * t7236;
    let t19359 = F::cast_from(0.25390814814814814815e1_f64) * t502 * t7271;
    let t19362 = F::new(0.701526e2) * t102 * t1563 * t19268;
    (t19349, t19351, t19355, t19357, t19359, t19362)
}
