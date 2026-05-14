//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 410/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk410<F: Float>(t154: F, t4129: F, t265: F, t952: F, t951: F, t243: F, t483: F, t242: F, t4103: F, t5: F, t12: F, t3: F, t963: F, t368: F, t142: F, t6: F) -> (F, F, F, F, F, F, F, F) {
    let t4130 = t4129 * t154;
    let t4132 = t952 * t265;
    let t4133 = t951 * t4132;
    let t4135 = t243 * t483;
    let t4136 = t242 * t4135;
    let t4138 = t5 * t4103;
    let t4140 = 1.0/pow_3_2(t12);
    let t4141 = t4140 * t3;
    let t4142 = t4141 * t154;
    let t4144 = t963 * t4132;
    let t4146 = t368 * t4135;
    let t4149 = t142 * t6 * t265;
    (t4130, t4133, t4136, t4138, t4142, t4144, t4146, t4149)
}
