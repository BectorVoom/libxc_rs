//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 411/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk411<F: Float>(t4221: F, t4275: F, t4130: F, t4133: F, t4136: F, t4138: F, t4142: F, t4144: F, t4146: F, t4149: F, t377: F, t364: F, t1076: F, t163: F, t158: F, t1080: F) -> (F, F, F, F) {
    let t4276 = t4221 * t4275;
    let t4287 = -0.25319e1 * t4130 + 0.16879333333333333333e1 * t4133 - 0.19692555555555555555e1 * t4136 - 0.93011851851851851854e0 * t4138 + 0.13651666666666666667e0 * t4142 - 0.27303333333333333333e0 * t4144 - 0.3185388888888888889e0 * t4146 - 0.36514074074074074075e0 * t4149;
    let t4288 = t4287 * t377;
    let t4290 = 1.0 * t364 * t4288;
    let t4292 = 1.0 / t1076 / t163;
    let t4293 = t158 * t4292;
    let t4294 = t4221 * t1080;
    (t4276, t4290, t4293, t4294)
}
