//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1095/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1095<F: Float>(t558: F, t8244: F, t41811: F, t41813: F, t1562: F, t8188: F, t41817: F, t41821: F, t275: F, t9596: F, t36646: F, t36663: F, t36674: F, t37921: F, t38029: F, t38031: F, t38036: F, t4041: F, t41796: F, t41803: F, t41808: F, t530: F, t884: F, t9302: F) -> (F, F) {
    let t43854 = t8244 * t558;
    let t43861 = F::new(0.39726959900411316772e-4) * t41811;
    let t43862 = F::new(0.11918087970123395032e-3) * t41813;
    let t43864 = F::new(0.4726e1) * t1562 * t8188;
    let t43868 = F::new(0.1440846329149835838e-2) * t41817;
    let t43869 = F::new(0.1440846329149835838e-2) * t41821;
    let t43874 = F::new(2.0) * t275 * t9596;
    let t43875 = F::new(0.11974241701863808564e0) * t4041 * t9302 + F::new(0.59871208509319042821e-1) * t884 * t43854 - F::new(0.23948483403727617128e0) * t36646 + F::new(0.212822999466489197e-4) * t41796 - F::new(0.212822999466489197e-4) * t41803 - F::new(0.1702583995731913576e-4) * t41808 - t43861 + t43862 - t43864 - F::new(0.2363e1) * t530 * t37921 + F::new(2.0) * t38029 + t43868 + t43869 + t38031 - F::new(0.39726959900411316772e-4) * t36663 - F::new(0.60975299583150056624e-3) * t36674 + F::new(2.0) * t38036 + t43874;
    (t43854, t43875)
}
