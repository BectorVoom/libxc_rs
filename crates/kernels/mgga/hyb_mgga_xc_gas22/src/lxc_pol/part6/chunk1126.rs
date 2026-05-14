//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1126/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1126<F: Float>(t21874: F, t221: F, t454: F, t2640: F, t7245: F, t2634: F, t2638: F, t1110: F, t21846: F, t2676: F, t2714: F, t2723: F, t2729: F, t1046: F, t2731: F, t7435: F) -> (F, F, F, F, F, F, F, F) {
    let t22141 = 0.11483599538271604938e-1 * t221 * t21874 * t454;
    let t22148 = t7245 * t2640;
    let t22150 = t2634 * t2634;
    let t22151 = 1.0 / t22150;
    let t22153 = t2638 * t2638;
    let t22154 = 1.0 / t22153;
    let t22157 = 0.91082604192152556044e5 * t1110 * t22151 * t21846 * t22154;
    let t22158 = t7245 * t2676;
    let t22162 = 36.0 * t2729 * t2714 * t2723;
    let t22166 = 0.64327917994770140268e2 * t2729 * t7435 * t2731 * t1046;
    (t22141, t22148, t22151, t22154, t22157, t22158, t22162, t22166)
}
