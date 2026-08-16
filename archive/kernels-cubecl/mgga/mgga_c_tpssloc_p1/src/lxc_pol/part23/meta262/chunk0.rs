//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 926/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk926<F: Float>(t225: F, t6364: F, t6435: F, t6362: F, t1390: F, t6463: F, t3701: F, t6324: F, t12461: F, t112: F, t6470: F, t9211: F, t9213: F, t9215: F, t9217: F, t9219: F, t9221: F, t9225: F) -> (F, F, F, F, F, F, F, F) {
    let t20029 = t6364 * t225;
    let t20044 = t6435 * t225;
    let t20060 = t6362 * t225;
    let t20067 = t6463 * t1390;
    let t20077 = t6324 * t3701;
    let t20085 = t6324 * t12461;
    let t20162 = t6470 * t112;
    let t20193 = -t9211 - t9213 - t9215 - t9217 - t9219 - t9221 - t9225;
    (t20029, t20044, t20060, t20067, t20077, t20085, t20162, t20193)
}
