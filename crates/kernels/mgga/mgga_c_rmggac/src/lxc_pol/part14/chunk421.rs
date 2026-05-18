//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 421/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk421<F: Float>(t4130: F, t4133: F, t4136: F, t4138: F, t4142: F, t4144: F, t4146: F, t4149: F, t410: F, t417: F, t431: F, t1037: F, t409: F) -> (F, F, F) {
    let t4151 = -F::new(0.34523333333333333333e1) * t4130 + F::new(0.23015555555555555556e1) * t4133 - F::new(0.26851481481481481482e1) * t4136 - F::new(0.93932222222222222223e0) * t4138 + F::new(0.73355e-1) * t4142 - F::new(0.14671e0) * t4144 - F::new(0.17116166666666666667e0) * t4146 - F::new(0.36793333333333333333e0) * t4149;
    let t4153 = t410 * t4151 * t417;
    let t4155 = F::new(0.5848223622634646207e0) * t431 * t4153;
    let t4157 = F::new(1.0) / t1037 / t409;
    (t4151, t4155, t4157)
}
