//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 880/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk880<F: Float>(t1648: F, t2622: F, t2572: F, t7011: F, t4913: F, t2705: F, t422: F, t7194: F, t1620: F, t1812: F, t7527: F, t1882: F, t2790: F) -> (F, F, F, F, F, F) {
    let t7605 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1648 * t2622;
    let t7607 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7011 * t2572;
    let t7609 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t4913 * t2572;
    let t7610 = t2705 * t422;
    let t7611 = t7194 * t7610;
    let t7613 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1620 * t7611;
    let t7615 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7527 * t1812;
    let t7617 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2790 * t1882;
    (t7605, t7607, t7609, t7613, t7615, t7617)
}
