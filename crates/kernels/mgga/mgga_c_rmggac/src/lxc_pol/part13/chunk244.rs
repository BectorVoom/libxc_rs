//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 244/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk244<F: Float>(t362: F, t135: F, t150: F, t1091: F) -> (F, F, F, F, F, F, F) {
    let t1105 = t362 * t362;
    let t1106 = 1.0 / t1105;
    let t1107 = t135 * t1106;
    let t1108 = t150 * t150;
    let t1109 = 1.0 / t1108;
    let t1110 = t1091 * t1109;
    let t1112 = 0.16081979498692535067e2 * t1107 * t1110;
    (t1105, t1106, t1107, t1108, t1109, t1110, t1112)
}
