//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 898/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk898<F: Float>(t3136: F, t3152: F, t898: F, t2464: F, t3932: F, t237: F, t3801: F) -> (F, F, F, F) {
    let t9756 = t3152 * t3136;
    let t9758 = F::new(0.23392894490538584828e1) * t898 * t9756;
    let t9759 = t3932 * t2464;
    let t9762 = t237 * t3801;
    (t9756, t9758, t9759, t9762)
}
