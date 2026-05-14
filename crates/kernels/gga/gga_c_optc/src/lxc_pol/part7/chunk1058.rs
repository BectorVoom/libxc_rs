//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1058/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1058<F: Float>(t23984: F, t2433: F, t176: F, t8378: F, t998: F, t2364: F, t7278: F, t2562: F, t7274: F, t999: F, t2367: F, t7258: F, t2543: F, t2550: F, t2360: F, t2368: F, t7285: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23985 = t2433 * t23984;
    let t23990 = t176 * t8378 * t998;
    let t23993 = t2364 * t7278;
    let t23996 = t999 * t7274 * t2562;
    let t23999 = t999 * t2367 * t7258;
    let t24003 = t999 * t7274 * t2543;
    let t24006 = t999 * t7274 * t2550;
    let t24008 = t2360 * t7278;
    let t24014 = t7285 * t2368;
    (t23985, t23990, t23993, t23996, t23999, t24003, t24006, t24008, t24014)
}
