//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 523/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk523<F: Float>(t1027: F, t617: F, t1809: F, t1620: F, t572: F, t995: F, t418: F, t1821: F, t1820: F, t1000: F, t562: F, t1037: F, t1627: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2575 = t1027 * t617;
    let t2576 = t1809 * t2575;
    let t2578 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1620 * t2576;
    let t2579 = t995 * t572;
    let t2580 = t2579 * t418;
    let t2581 = t1821 * t2580;
    let t2583 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1820 * t2581;
    let t2584 = t1000 * t562;
    let t2585 = t1821 * t2584;
    let t2587 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1820 * t2585;
    let t2590 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1627 * t1037;
    (t2575, t2576, t2578, t2579, t2580, t2581, t2583, t2584, t2585, t2587, t2590)
}
