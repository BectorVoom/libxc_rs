//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 813/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk813<F: Float>(t1686: F, t2046: F, t2050: F, t31: F, t2131: F, t5321: F, t38350: F, t7473: F, t34884: F, t9046: F, t2289: F, t34881: F) -> (F, F, F, F, F) {
    let t39808 = t2046 * t2050 * t1686 * t31;
    let t39809 = F::new(0.43368970657079495312e-4) * t39808;
    let t39827 = F::new(0.4726e1) * t5321 * t2131;
    let t39832 = t38350 * t7473;
    let t39840 = t34884 * t9046;
    let t39841 = F::new(0.24829349937757072982e-4) * t39840;
    let t39842 = t34881 * t2289;
    (t39809, t39827, t39832, t39841, t39842)
}
