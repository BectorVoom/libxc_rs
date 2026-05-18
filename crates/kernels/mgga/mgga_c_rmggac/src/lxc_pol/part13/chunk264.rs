//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 264/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk264<F: Float>(t1212: F, t209: F, t469: F, t6: F, t1183: F, t77: F, t9: F, t31: F, t212: F, t222: F, t1189: F) -> (F, F, F, F, F, F) {
    let t1215 = t469 * t6 * t1212 * t209;
    let t1219 = t469 * t1183 * t209;
    let t1223 = F::new(1.0) / t9 / t77;
    let t1224 = t31 * t1223;
    let t1227 = F::new(0.21341877202031537856e0) * t212 * t1224 * t222;
    let t1228 = t212 * t1189;
    (t1215, t1219, t1223, t1224, t1227, t1228)
}
