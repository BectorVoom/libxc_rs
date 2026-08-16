//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1397/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1397<F: Float>(t1442: F, t52330: F, t52331: F, t19: F, t5328: F, t8974: F, t4356: F, t3119: F, t55127: F, t5311: F, t3104: F, t438: F, t935: F) -> (F, F, F, F, F, F, F) {
    let t58917 = t52330 * t52331 * t1442;
    let t58922 = t5328 * t19;
    let t58923 = t58922 * t8974;
    let t58928 = t58922 * t4356;
    let t58932 = t55127 * t3119;
    let t58941 = t5311 * t5311;
    let t58942 = t3104 * t58941;
    let t58947 = t5328 * t935 * t438;
    (t58917, t58923, t58928, t58932, t58941, t58942, t58947)
}
