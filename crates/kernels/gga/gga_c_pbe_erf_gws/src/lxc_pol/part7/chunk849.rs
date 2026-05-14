//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 849/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk849<F: Float>(t1620: F, t1621: F, t5097: F, t617: F, t649: F, t1672: F, t1794: F, t211: F, t5105: F, t633: F, t1724: F, t5500: F, t1640: F, t1643: F, t16986: F, t639: F) -> (F, F, F, F, F) {
    let t17359 = 16.0 / 15.0 * t1620 * t1621 * t649 * t5097 * t617;
    let t17361 = t211 * t1672 * t1794;
    let t17362 = 16.0 / 45.0 * t17361;
    let t17363 = t633 * t5105;
    let t17364 = 32.0 / 15.0 * t17363;
    let t17368 = 8.0 / 5.0 * t1620 * t1621 * t5500 * t1724;
    let t17372 = 4.0 / 9.0 * t639 * t1640 * t1643 * t16986;
    (t17359, t17362, t17364, t17368, t17372)
}
