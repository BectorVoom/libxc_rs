//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 427/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk427<F: Float>(t158: F, t4272: F, t1079: F, t166: F, t4221: F, t4130: F, t4133: F, t4136: F, t4138: F, t4142: F, t4144: F, t4146: F, t4149: F) -> (F, F, F) {
    let t4273 = t158 * t4272;
    let t4275 = F::new(1.0) / t1079 / t166;
    let t4276 = t4221 * t4275;
    let t4287 = -F::new(0.25319e1) * t4130 + F::new(0.16879333333333333333e1) * t4133 - F::new(0.19692555555555555555e1) * t4136 - F::new(0.93011851851851851854e0) * t4138 + F::new(0.13651666666666666667e0) * t4142 - F::new(0.27303333333333333333e0) * t4144 - F::new(0.3185388888888888889e0) * t4146 - F::new(0.36514074074074074075e0) * t4149;
    (t4273, t4276, t4287)
}
