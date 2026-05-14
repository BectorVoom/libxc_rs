//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 785/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk785<F: Float>(t2264: F, t2269: F, t3017: F, t3059: F, t3732: F, t3744: F, t3748: F, t3752: F, t3754: F, t3759: F, t3763: F, t871: F) -> (F, F) {
    let t3792 = -0.17648625e1 * t3744 + 0.3529725e1 * t3748 + t2264 - 0.103295e1 * t3017 + 0.1549425e1 * t3732 + 0.31558125e0 * t3752 + 0.6311625e0 * t3754 + t2269 - 0.41678e0 * t3059 + 0.312585e0 * t3759 + 0.312585e0 * t3763;
    let t3793 = t3792 * t871;
    (t3792, t3793)
}
