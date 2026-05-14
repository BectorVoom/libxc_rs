//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 738/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk738<F: Float>(t40250: F, t1652: F, t2084: F, t2145: F, t27: F, t16156: F, t9213: F, t1965: F, t1967: F, t28: F, t8511: F, t1562: F, t7399: F, t118: F, t1986: F, t352: F, t39866: F) -> (F, F, F, F, F, F) {
    let t40251 = 0.24829349937757072982e-4 * t40250;
    let t40259 = t2145 * t27 * t2084 * t1652;
    let t40260 = 0.18183107769496894486e-1 * t40259;
    let t40262 = t16156 * t9213;
    let t40263 = 0.39726959900411316772e-4 * t40262;
    let t40278 = t8511 * t1965 * t1967 * t28;
    let t40294 = 0.4726e1 * t1562 * t7399;
    let t40313 = t1986 * t118 * t39866 * t352;
    (t40251, t40260, t40263, t40278, t40294, t40313)
}
