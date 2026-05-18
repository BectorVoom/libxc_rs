//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1142/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1142<F: Float>(t1276: F, t2938: F, t3366: F, t11882: F, t23498: F, t263: F, t2928: F, t40815: F, t826: F, t11880: F, t11881: F, t2391: F) -> (F, F, F, F) {
    let t42508 = t1276 * t3366 * t2938;
    let t42512 = t23498 * param_eta * t11882;
    let t42516 = t40815 * t263 * t2928 * t826;
    let t42519 = t11880 * t11881 * t2391;
    (t42508, t42512, t42516, t42519)
}
