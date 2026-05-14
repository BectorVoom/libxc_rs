//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 716/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk716<F: Float>(t12729: F, t2221: F, t1643: F, t3445: F, t9115: F, t2157: F, t920: F, t2211: F, t2210: F, t11982: F, t3434: F, t160: F, t7800: F, t11437: F, t3439: F, t1047: F, t1637: F, t89: F) -> (F, F, F, F, F, F) {
    let t12730 = t2221 * t12729;
    let t12733 = t3445 * t1643;
    let t12734 = t9115 * t12733;
    let t12737 = t920 * t2157;
    let t12738 = t2211 * t12737;
    let t12739 = t2210 * t12738;
    let t12742 = t3434 * t11982;
    let t12743 = t2210 * t12742;
    let t12746 = t160 * t7800;
    let t12747 = t12746 * t11437;
    let t12748 = t3439 * t12747;
    let t12752 = t89 * t1637 * t1047;
    (t12730, t12734, t12739, t12743, t12748, t12752)
}
