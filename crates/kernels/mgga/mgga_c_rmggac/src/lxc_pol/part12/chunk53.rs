//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 53/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk53<F: Float>(t4: F, t140: F, t34: F, t6: F, t12: F, t13: F, t138: F, t135: F, t77: F) -> (F, F, F, F, F, F, F, F) {
    let t141 = t4 * t4;
    let t142 = t140 * t141;
    let t145 = t142 * t6 / t34;
    let t147 = 0.379785e1 * t13 + 0.8969e0 * t12 + 0.204775e0 * t138 + 0.123235e0 * t145;
    let t150 = 1.0 + 0.16081979498692535067e2 / t147;
    let t151 = f64::ln(t150);
    let t153 = 0.621814e-1 * t135 * t151;
    let t154 = 1.0 / t77;
    (t141, t142, t145, t147, t150, t151, t153, t154)
}
