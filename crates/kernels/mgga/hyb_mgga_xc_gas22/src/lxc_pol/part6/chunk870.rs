//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 870/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk870<F: Float>(t7237: F, t7238: F, t7241: F, t1110: F, t2696: F, t483: F, t1112: F, t1096: F, t2635: F, t2634: F, t488: F) -> (F, F, F, F, F, F, F) {
    let t7242 = t7237 * t7238 * t7241;
    let t7244 = F::new(0.10254018858216406658e4) * t1110 * t7242;
    let t7245 = t2696 * t483;
    let t7246 = t7245 * t1112;
    let t7249 = t2635 * t7238 * t1096;
    let t7251 = F::new(0.35089341735807877242e1) * t1110 * t7249;
    let t7253 = F::new(1.0) / t2634 / t488;
    (t7242, t7244, t7245, t7246, t7249, t7251, t7253)
}
