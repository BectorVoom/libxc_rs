//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 590/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk590<F: Float>(t2773: F, t2791: F, t1089: F, t567: F, t2647: F, t483: F, t1096: F, t2636: F, t2674: F, t2635: F) -> (F, F, F, F, F, F) {
    let t2792 = t2773 * t2791;
    let t2798 = t567 * t1089;
    let t2802 = t483 * t2647;
    let t2803 = t2636 * t1096;
    let t2806 = t2674 * t1096;
    let t2809 = t483 * t2635;
    (t2792, t2798, t2802, t2803, t2806, t2809)
}
