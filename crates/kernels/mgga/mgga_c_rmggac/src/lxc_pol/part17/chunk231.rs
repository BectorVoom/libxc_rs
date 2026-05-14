//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 231/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk231<F: Float>(t954: F, t957: F, t960: F, t964: F, t966: F, t969: F, t377: F, t364: F) -> (F, F, F) {
    let t1101 = -0.42198333333333333333e0 * t954 + 0.84396666666666666666e0 * t957 + 0.39862222222222222223e0 * t960 + 0.68258333333333333333e-1 * t964 + 0.13651666666666666667e0 * t966 + 0.13692777777777777778e0 * t969;
    let t1102 = t1101 * t377;
    let t1104 = 1.0 * t364 * t1102;
    (t1101, t1102, t1104)
}
