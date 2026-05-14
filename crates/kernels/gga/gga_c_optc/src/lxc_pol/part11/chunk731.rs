//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 731/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk731<F: Float>(t1544: F, t2667: F, t465: F, t8113: F, t1506: F, t8459: F, t1027: F, t8446: F, t10990: F, t1135: F, t2639: F) -> (F, F, F, F, F, F, F) {
    let t11940 = t1544 * t2667;
    let t11943 = t465 * t8113;
    let t11962 = t8459 * t1506;
    let t11975 = t8446 * t1027;
    let t11982 = t465 * t10990;
    let t12002 = t1135 * t1506;
    let t12026 = t1544 * t2639;
    (t11940, t11943, t11962, t11975, t11982, t12002, t12026)
}
