//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 997/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk997<F: Float>(t2567: F, t481: F, t2148: F, t7614: F, t921: F, t538: F, t6155: F, t2162: F, t503: F) -> (F, F, F, F, F, F, F) {
    let t7615 = t2567 * t481;
    let t7616 = t2148 * t7615;
    let t7618 = 0.13972381860938637374e0 * t7614 * t7616;
    let t7619 = t921 * t481;
    let t7620 = t538 * t7619;
    let t7622 = 0.10975748638225852664e-1 * t6155 * t7620;
    let t7623 = t503 * t2162;
    (t7615, t7616, t7618, t7619, t7620, t7622, t7623)
}
