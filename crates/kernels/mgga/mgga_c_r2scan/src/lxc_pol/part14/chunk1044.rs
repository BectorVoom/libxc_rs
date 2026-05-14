//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1044/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1044<F: Float>(t10831: F, t1102: F, t3692: F, t1543: F, t3582: F, t2333: F, t2526: F, t795: F, t3446: F, t3453: F, t7104: F, t10655: F, t11603: F, t10922: F, t11572: F, t3308: F, t3429: F) -> (F, F, F, F, F, F, F) {
    let t40485 = t1102 * t10831 * t3692;
    let t40487 = t3582 * t1543;
    let t40491 = t2333 * t2526;
    let t40492 = t40491 * t795;
    let t40511 = t3446 * t3453 * t7104;
    let t40513 = t10655 * t11603;
    let t40515 = t10922 * t11603;
    let t40518 = t3429 * t3308 * t11572;
    (t40485, t40487, t40492, t40511, t40513, t40515, t40518)
}
