//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1104/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1104(t26930: f64, t6720: f64, t1096: f64, t6724: f64, t1021: f64, t6728: f64, t6732: f64, t29045: f64, t29047: f64, t29049: f64, t29052: f64, t29054: f64, t29057: f64, t29060: f64, t29063: f64, t29065: f64, t29067: f64, t29069: f64, t29071: f64) -> (f64, f64, f64, f64, f64) {
    let t29073 = t26930 * t6720;
    let t29075 = t1096 * t6724;
    let t29077 = t1021 * t6728;
    let t29079 = t1021 * t6732;
    let t29081 = t29045 / 16.0_f64 - t29047 / 8.0_f64 + t29049 / 12.0_f64 + t29052 / 8.0_f64 - t29054 / 12.0_f64 - t29057 / 16.0_f64 - t29060 / 72.0_f64 + t29063 / 24.0_f64 - t29065 / 128.0_f64 + t29067 / 64.0_f64 - t29069 / 48.0_f64 - t29071 / 64.0_f64 + t29073 / 48.0_f64 + t29075 / 128.0_f64 - t29077 / 288.0_f64 - t29079 / 96.0_f64;
    (t29073, t29075, t29077, t29079, t29081)
}
