//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 721/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk721<F: Float>(t1123: F, t1539: F, t1145: F, t1129: F, t1535: F, t1117: F, t513: F, t1118: F, t1530: F) -> (F, F, F, F, F, F, F, F) {
    let t3697 = t1539 * t1123;
    let t3698 = t1145 * t3697;
    let t3701 = t1539 * t1129;
    let t3702 = t1145 * t3701;
    let t3705 = t1535 * t1129;
    let t3706 = t1145 * t3705;
    let t3713 = t1117 * t513;
    let t3714 = t1118 * t1530;
    (t3697, t3698, t3701, t3702, t3705, t3706, t3713, t3714)
}
