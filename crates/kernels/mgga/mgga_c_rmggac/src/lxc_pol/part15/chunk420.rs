//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 420/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk420<F: Float>(t4132: F, t951: F, t243: F, t483: F, t242: F, t4103: F, t5: F, t12: F, t3: F, t154: F, t963: F, t368: F) -> (F, F, F, F, F, F) {
    let t4133 = t951 * t4132;
    let t4135 = t243 * t483;
    let t4136 = t242 * t4135;
    let t4138 = t5 * t4103;
    let t4140 = F::new(1.0)/pow_3_2::<f64>(t12);
    let t4141 = t4140 * t3;
    let t4142 = t4141 * t154;
    let t4144 = t963 * t4132;
    let t4146 = t368 * t4135;
    (t4133, t4136, t4138, t4142, t4144, t4146)
}
