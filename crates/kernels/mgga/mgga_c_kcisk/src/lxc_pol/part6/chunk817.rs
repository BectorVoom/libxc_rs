//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 817/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk817<F: Float>(t23460: F, t23606: F, t23609: F, t29082: F, t29085: F, t29091: F, t29097: F, t29152: F, t29155: F, t29161: F, t29164: F, t29166: F, t29168: F, t29170: F, t29211: F, t1725: F) -> (F,) {
    let t29226 = -0.33218518518518518518e0 * t29082 + 0.11958666666666666667e1 * t29085 - 0.17938e1 * t29091 - 0.29896666666666666667e0 * t29097 - 0.73028148148148148146e-1 * t29152 - 0.16431333333333333333e0 * t29155 + 0.19931111111111111111e0 * t23460 + 0.10954222222222222222e0 * t23606 + 0.32862666666666666666e0 * t23609 + 0.32862666666666666666e0 * t29161 - 0.98587999999999999998e0 * t29164 + 0.3071625e0 * t29166 + 0.46074375e0 * t29168 - 0.28483875e1 * t29170;
    let t29227 = t29211 + t29226;
    let t29228 = t29227 * t1725;
    (t29228,)
}
