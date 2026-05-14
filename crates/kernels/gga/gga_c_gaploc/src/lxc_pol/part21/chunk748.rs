//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 748/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk748<F: Float>(t1589: F, t2586: F, t2027: F, t2628: F, t6138: F, t959: F, t5673: F, t2603: F, t549: F, t2022: F, t1984: F, t7426: F) -> (F, F, F, F, F, F) {
    let t7558 = t1589 * t2586;
    let t7563 = t2027 * t2628;
    let t7565 = t6138 * t959;
    let t7567 = t5673 * t959;
    let t7569 = t549 * t2603;
    let t7570 = t2022 * t7569;
    let t7572 = t1984 * t7426;
    (t7558, t7563, t7565, t7567, t7570, t7572)
}
