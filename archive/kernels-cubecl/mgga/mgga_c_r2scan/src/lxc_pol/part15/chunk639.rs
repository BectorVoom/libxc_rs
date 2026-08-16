//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 639/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk639<F: Float>(t3263: F, t3574: F, t3262: F, t106: F, t920: F, t97: F) -> (F, F, F, F) {
    let t3575 = t3263 * t3574;
    let t3576 = t3262 * t3575;
    let t3577 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3576;
    let t3578 = t106 * t920;
    let t3579 = t97 * t3578;
    (t3575, t3577, t3578, t3579)
}
