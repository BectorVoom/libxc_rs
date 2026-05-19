//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 424/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk424<F: Float>(t1105: F, t362: F, t135: F, t1091: F, t376: F, t1108: F, t150: F, t147: F, t1109: F, t1062: F, t401: F, t402: F) -> (F, F, F, F, F) {
    let t4207 = F::new(1.0) / t1105 / t362;
    let t4208 = t135 * t4207;
    let t4209 = t1091 * t376;
    let t4211 = F::new(1.0) / t1108 / t150;
    let t4212 = t4209 * t4211;
    let t4214 = F::cast_from(0.51726012919273400301e3_f64) * t4208 * t4212;
    let t4216 = F::new(1.0) / t1105 / t147;
    let t4217 = t135 * t4216;
    let t4218 = t4209 * t1109;
    let t4220 = F::cast_from(0.96491876992155210402e2_f64) * t4217 * t4218;
    let t4221 = t1062 * t401;
    let t4222 = t4221 * t402;
    (t4209, t4214, t4220, t4221, t4222)
}
