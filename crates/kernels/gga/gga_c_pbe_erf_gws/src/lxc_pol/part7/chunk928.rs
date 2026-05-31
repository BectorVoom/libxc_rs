//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 928/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk928<F: Float>(t1621: F, t1791: F, t5097: F, t639: F, t661: F, t1620: F, t617: F, t649: F, t1672: F, t1794: F, t211: F, t5105: F, t633: F) -> (F, F, F, F) {
    let t17354 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t639 * t1621 * t1791 * t5097 * t661;
    let t17359 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1620 * t1621 * t649 * t5097 * t617;
    let t17361 = t211 * t1672 * t1794;
    let t17362 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17361;
    let t17363 = t633 * t5105;
    (t17354, t17359, t17362, t17363)
}
