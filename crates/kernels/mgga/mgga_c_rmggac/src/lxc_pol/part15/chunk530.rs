//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 530/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk530<F: Float>(t640: F, t7556: F, t7555: F, t7553: F, t27: F, t3118: F, t684: F, t2123: F, t874: F, t36: F) -> (F, F, F, F, F, F, F) {
    let t7557 = t640 * t7556;
    let t7558 = t7555 * t7557;
    let t7559 = t7553 * t7558;
    let t7561 = t27 * t3118;
    let t7562 = t684 * t7561;
    let t7567 = t874 * t2123;
    let t7577 = t874 * t36;
    (t7557, t7558, t7559, t7561, t7562, t7567, t7577)
}
