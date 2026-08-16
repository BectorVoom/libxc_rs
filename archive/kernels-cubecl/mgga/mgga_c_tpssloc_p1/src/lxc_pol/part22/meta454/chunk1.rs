//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1820/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1820<F: Float>(t9211: F, t9213: F, t9215: F, t9217: F, t9219: F, t9221: F, t9225: F, t1437: F, t5389: F, t5445: F, t1864: F, t5398: F) -> (F, F, F, F) {
    let t20193 = -t9211 - t9213 - t9215 - t9217 - t9219 - t9221 - t9225;
    let t20201 = t5389 * t1437;
    let t20204 = t1437 * t5445;
    let t20207 = t1864 * t5398;
    (t20193, t20201, t20204, t20207)
}
