//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1112/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1112<F: Float>(t1575: F, t2097: F, t571: F, t6359: F, t774: F, t255: F, t1541: F, t2182: F, t2168: F, t6448: F, t489: F, t5134: F, t548: F, t524: F, t6238: F, t20473: F, t503: F) -> (F, F, F, F, F, F, F, F) {
    let t20664 = t571 * t1575 * t2097;
    let t20670 = t6359 * t774;
    let t20672 = t571 * t20670 * t255;
    let t20698 = t2182 * t1541;
    let t20705 = t6448 * t2168;
    let t20720 = t5134 * t489;
    let t20721 = t20720 * t548;
    let t20758 = t524 * t6238 * t489;
    let t20762 = t503 * t20473;
    (t20664, t20672, t20698, t20705, t20720, t20721, t20758, t20762)
}
