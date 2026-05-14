//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 722/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk722<F: Float>(t7279: F, t8365: F, t2283: F, t7921: F, t2185: F, t8675: F, t1997: F, t1540: F, t880: F, t1347: F, t2406: F, t16156: F, t9096: F, t8812: F, t7269: F, t8368: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38946 = t8365 * t7279;
    let t38965 = t7921 * t2283;
    let t38967 = t8675 * t2185;
    let t38968 = t38967 * t1997;
    let t38969 = 0.24829349937757072982e-4 * t38968;
    let t38973 = t1540 * t880;
    let t38976 = t1347 * t2406;
    let t38986 = t16156 * t9096;
    let t38998 = t16156 * t8812;
    let t39023 = t8368 * t7269;
    (t38946, t38965, t38967, t38969, t38973, t38976, t38986, t38998, t39023)
}
