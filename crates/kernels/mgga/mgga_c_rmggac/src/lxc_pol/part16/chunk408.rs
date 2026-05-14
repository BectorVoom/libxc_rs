//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 408/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk408<F: Float>(t4209: F, t4211: F, t4208: F, t1105: F, t147: F, t135: F, t1109: F, t1062: F, t401: F, t402: F, t1054: F, t1089: F, t1092: F, t245: F, t977: F, t214: F, t410: F) -> (F, F, F, F, F, F, F) {
    let t4212 = t4209 * t4211;
    let t4214 = 0.51726012919273400301e3 * t4208 * t4212;
    let t4216 = 1.0 / t1105 / t147;
    let t4217 = t135 * t4216;
    let t4218 = t4209 * t1109;
    let t4220 = 0.96491876992155210402e2 * t4217 * t4218;
    let t4221 = t1062 * t401;
    let t4222 = t4221 * t402;
    let t4232 = 0.10685e0 * t1054 * t245 * t1089 * t1092;
    let t4233 = t245 * t977;
    let t4237 = t214 * t410;
    (t4214, t4220, t4221, t4222, t4232, t4233, t4237)
}
