//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 917/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk917<F: Float>(t16210: F, t6913: F, t1048: F, t23: F, t6: F, t161: F, t6916: F, t3269: F, t9: F, t7: F, t171: F, t6919: F, t1846: F, t2063: F, t11612: F, t11634: F, t220: F) -> (F, F, F, F, F, F) {
    let t16211 = t16210 * t6913;
    let t16214 = 1.0 / t23 / t1048;
    let t16215 = t6 * t16214;
    let t16216 = t161 * t16215;
    let t16217 = t16216 * t6916;
    let t16220 = 1.0 / t9 / t3269;
    let t16221 = t7 * t16220;
    let t16222 = t171 * t16221;
    let t16223 = t16222 * t6919;
    let t16225 = t1846 * t2063;
    let t16227 = t11612 * t2063;
    let t16229 = t11634 * t220;
    (t16211, t16217, t16223, t16225, t16227, t16229)
}
