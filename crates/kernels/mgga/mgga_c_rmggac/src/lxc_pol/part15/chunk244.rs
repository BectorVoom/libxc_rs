//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 244/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk244<F: Float>(t198: F, t673: F, t1193: F, t209: F, t476: F, t1156: F, t23: F, t77: F, t9: F, t31: F, t212: F, t222: F) -> (F, F, F, F, F, F, F) {
    let t1194 = t673 * t198;
    let t1195 = t1193 * t1194;
    let t1196 = t476 * t209;
    let t1205 = t23 * t1156;
    let t1223 = F::new(1.0) / t9 / t77;
    let t1224 = t31 * t1223;
    let t1227 = F::new(0.21341877202031537856e0) * t212 * t1224 * t222;
    (t1194, t1195, t1196, t1205, t1223, t1224, t1227)
}
