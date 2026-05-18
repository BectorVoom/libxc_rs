//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 867/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk867<F: Float>(t2522: F, t360: F, t2530: F, t978: F, t2559: F, t974: F, t2593: F, t2601: F, t6966: F, t2569: F, t260: F, t1088: F, t2634: F) -> (F, F, F, F, F, F, F, F) {
    let t7150 = F::new(1.0) / t2522 / t360;
    let t7154 = t2530 * t978;
    let t7159 = t974 * t2559;
    let t7165 = t2593 * t2601;
    let t7176 = F::new(0.55403703703703703703e-1) * t6966;
    let t7183 = F::new(0.53272592592592592592e-1) * t6966;
    let t7222 = t260 * t2569;
    let t7237 = F::new(1.0) / t2634 / t1088;
    (t7150, t7154, t7159, t7165, t7176, t7183, t7222, t7237)
}
