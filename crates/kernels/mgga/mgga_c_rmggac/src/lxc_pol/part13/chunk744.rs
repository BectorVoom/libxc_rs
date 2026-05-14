//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 744/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk744<F: Float>(t16043: F, t9194: F, t9198: F, t2286: F, t35277: F, t1587: F, t236: F, t3352: F, t495: F, t7230: F, t1528: F, t3351: F, t498: F, t9210: F, t321: F, t7248: F) -> (F, F, F, F, F, F) {
    let t38574 = t16043 * t9194;
    let t38576 = t16043 * t9198;
    let t38578 = t35277 * t2286;
    let t38583 = t7230 * t3352 * t236 * t1587 * t495;
    let t38588 = t3351 * t9210 * t236 * t1528 * t498;
    let t38594 = t3351 * t7248 * t236 * t1528 * t321;
    (t38574, t38576, t38578, t38583, t38588, t38594)
}
