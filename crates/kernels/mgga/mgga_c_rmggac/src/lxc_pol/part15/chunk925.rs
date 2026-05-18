//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 925/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk925<F: Float>(t262: F, t45418: F, t7198: F, t1469: F, t2318: F, t34976: F, t40145: F, t2281: F, t35039: F, t39851: F, t16504: F, t552: F) -> (F, F, F, F, F) {
    let t45419 = t262 * t45418;
    let t45420 = t7198 * t45419;
    let t45424 = t40145 * t34976 * t2318 * t1469;
    let t45428 = t39851 * t35039 * t2281 * t1469;
    let t45432 = t39851 * t16504 * t552 * t1469;
    (t45419, t45420, t45424, t45428, t45432)
}
