//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 405/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk405<F: Float>(t242: F, t4135: F, t4103: F, t5: F, t12: F, t3: F, t154: F, t4132: F, t963: F, t368: F, t142: F, t265: F, t6: F, t4130: F, t4133: F, t410: F, t417: F) -> (F, F, F, F, F, F, F, F) {
    let t4136 = t242 * t4135;
    let t4138 = t5 * t4103;
    let t4140 = 1.0/pow_3_2(t12);
    let t4141 = t4140 * t3;
    let t4142 = t4141 * t154;
    let t4144 = t963 * t4132;
    let t4146 = t368 * t4135;
    let t4149 = t142 * t6 * t265;
    let t4151 = -0.34523333333333333333e1 * t4130 + 0.23015555555555555556e1 * t4133 - 0.26851481481481481482e1 * t4136 - 0.93932222222222222223e0 * t4138 + 0.73355e-1 * t4142 - 0.14671e0 * t4144 - 0.17116166666666666667e0 * t4146 - 0.36793333333333333333e0 * t4149;
    let t4153 = t410 * t4151 * t417;
    (t4136, t4138, t4142, t4144, t4146, t4149, t4151, t4153)
}
