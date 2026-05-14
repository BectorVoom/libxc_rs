//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1237/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1237<F: Float>(t10862: F, t10878: F, t1234: F, t1312: F, t136: F, t2015: F, t2163: F, t2165: F, t26: F, t2966: F, t2967: F, t30153: F, t30156: F, t30158: F, t30167: F, t30170: F, t30172: F, t30174: F, t30176: F, t30179: F, t30181: F, t3304: F, t4109: F, t4111: F, t4115: F, t676: F, t765: F, t8846: F, t8872: F) -> (F,) {
    let t30197 = t30153 / 96.0 + t30156 / 96.0 + t30158 / 96.0 + 3.0 / 8.0 * t2966 * t2967 * t3304 - 3.0 / 32.0 * t676 * t10878 - 3.0 / 32.0 * t1234 * t8872 - t30167 / 32.0 - t30170 / 32.0 - t30172 / 32.0 - t30174 / 32.0 - t30176 / 32.0 - t30179 / 32.0 - t30181 / 16.0 - 3.0 / 32.0 * t10862 * t765 - 3.0 / 64.0 * t4115 * t2165 - 3.0 / 64.0 * t136 * t26 * t2163 * t4109 - 3.0 / 32.0 * t136 * t26 * t8846 * t1312 - 3.0 / 64.0 * t2015 * t4111;
    (t30197,)
}
