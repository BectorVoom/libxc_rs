//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 939/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk939<F: Float>(t11826: F, t11841: F, t11858: F, t12213: F, t12214: F, t12215: F, t12216: F, t12217: F, t12218: F, t12219: F, t12220: F, t12221: F, t12222: F, t12224: F, t12225: F, t12226: F, t12228: F, t12229: F, t12230: F, t12231: F) -> (F,) {
    let t12637 = t12213 - t12214 - t12215 + t12216 - t12217 + t12218 + t12219 - t12220 + t12221 - t12222 + 0.53968515702149165444e-6 * t11826 - t12224 + t12225 - t12226 - 0.57970906942607043475e-5 * t11841 + t12228 - t12229 + t12230 - t12231 - 0.12650553385416666667e-5 * t11858;
    (t12637,)
}
