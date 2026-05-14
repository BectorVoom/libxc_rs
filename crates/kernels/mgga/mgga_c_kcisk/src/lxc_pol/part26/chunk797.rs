//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 797/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk797<F: Float>(t12558: F, t14: F, t2886: F, t31: F, t12514: F, t2917: F, t52: F, t12535: F, t2921: F, t846: F, t2912: F, t2918: F, t157: F, t2903: F, t2856: F, t2879: F, t831: F) -> (F, F, F, F, F, F) {
    let t12559 = t14 * t12558;
    let t12561 = 1.0 / t2886 / t31;
    let t12562 = t12514 * t12561;
    let t12564 = 0.51725014705706168417e3 * t12559 * t12562;
    let t12566 = 1.0 / t2917 / t52;
    let t12568 = t12566 * t12535 * t2921;
    let t12572 = t2921 * t846;
    let t12573 = t2918 * t2912 * t12572;
    let t12576 = t157 * t2903;
    let t12581 = 6.0 * t2856 * t831 * t2879;
    (t12564, t12566, t12568, t12573, t12576, t12581)
}
