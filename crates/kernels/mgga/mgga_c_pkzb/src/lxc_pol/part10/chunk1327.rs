//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1327/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1327<F: Float>(t237: F, t26070: F, t26141: F, t26157: F, t26193: F, t26218: F, t26263: F, t26313: F, t26360: F, t2860: F, t7536: F, t20637: F, t25892: F, t2852: F, t7528: F, t1108: F, t20671: F) -> (F, F, F, F, F) {
    let t26364 = t237 * (t26070 + t26141 + t26157 + t26193 + t26218 + t26263 + t26313 + t26360);
    let t26366 = 0.69263436422725855034e2 * t2860 * t7536;
    let t26369 = 0.4155806185363551302e3 * t20637 * t2852 * t25892;
    let t26371 = 0.46785788981077169656e1 * t2860 * t7528;
    let t26374 = 0.14035736694323150897e2 * t20671 * t1108 * t25892;
    (t26364, t26366, t26369, t26371, t26374)
}
