//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1631/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1631(t225: f64, t6151: f64, t6153: f64, t6239: f64, t1720: f64, t5052: f64, t1751: f64, t4940: f64, t18571: f64, t491: f64, t1252: f64, t14972: f64, t14980: f64, t15797: f64, t1761: f64, t3487: f64, t3593: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t6244: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19232 = t6151 * t225;
    let t19234 = t6153 * t225;
    let t19249 = t6239 * t225;
    let t19253 = t1720 * t5052;
    let t19256 = t4940 * t1751;
    let t19259 = t18571 * t491;
    let t19261 = -t1252 * t19232 - 2.0_f64 * t1252 * t19234 - t1252 * t19249 - 2.0_f64 * t14972 * t1761 - 2.0_f64 * t14980 * t1761 - 2.0_f64 * t15797 * t1761 + 2.0_f64 * t19253 * t498 + 2.0_f64 * t19256 * t498 + t19259 * t498 + 2.0_f64 * t3487 * t6244 + 2.0_f64 * t3593 * t6244 - 2.0_f64 * t4945 * t5089 - 2.0_f64 * t5055 * t5089;
    (t19232, t19234, t19249, t19253, t19256, t19259, t19261)
}
