//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 830/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk830<F: Float>(t1804: F, t1809: F, t6214: F, t1815: F, t765: F, t136: F, t2153: F, t550: F, t168: F, t693: F, t140: F, t35: F, t6007: F) -> (F, F, F, F, F, F, F) {
    let t6216 = t1804 * t6214 * t1809;
    let t6226 = t1815 * t765;
    let t6227 = t136 * t6226;
    let t6229 = t550 * t2153;
    let t6230 = t136 * t6229;
    let t6270 = F::new(1.0) / t168 / t693;
    let t6278 = F::new(14.0) / F::new(243.0) * t35 * t6007 * t140;
    (t6216, t6226, t6227, t6229, t6230, t6270, t6278)
}
