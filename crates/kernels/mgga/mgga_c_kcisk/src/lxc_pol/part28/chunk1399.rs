//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1399/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1399<F: Float>(t2568: F, t7400: F, t24232: F, t9704: F, t24445: F, t736: F, t24257: F, t33097: F, t70785: F, t748: F, t2454: F, t7336: F, t9705: F, t23052: F, t5290: F, t9708: F) -> (F, F, F, F, F, F, F) {
    let t122174 = t7400 * t2568;
    let t122176 = t9704 * t24232;
    let t122178 = t24445 * t736;
    let t122180 = t33097 * t24257;
    let t122182 = t70785 * t748;
    let t122184 = t7336 * t2454;
    let t122185 = t122184 * t9705;
    let t122188 = t9708 * t5290 * t23052;
    (t122174, t122176, t122178, t122180, t122182, t122185, t122188)
}
