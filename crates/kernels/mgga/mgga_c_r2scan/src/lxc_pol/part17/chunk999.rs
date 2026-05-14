//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 999/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk999<F: Float>(t2333: F, t2526: F, t10655: F, t11603: F, t10922: F, t11572: F, t3308: F, t3429: F, t10810: F, t3692: F, t10935: F, t2816: F, t3446: F, t10928: F, t122: F, t3434: F, t874: F, t955: F) -> (F, F, F, F, F, F, F) {
    let t40491 = t2333 * t2526;
    let t40513 = t10655 * t11603;
    let t40515 = t10922 * t11603;
    let t40518 = t3429 * t3308 * t11572;
    let t40556 = t3429 * t10810 * t3692;
    let t40559 = t3446 * t10935 * t2816;
    let t40564 = t3434 * t10928 * t955 * t874 * t122;
    (t40491, t40513, t40515, t40518, t40556, t40559, t40564)
}
