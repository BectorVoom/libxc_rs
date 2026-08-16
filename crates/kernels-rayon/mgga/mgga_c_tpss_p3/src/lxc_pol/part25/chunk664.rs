//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 664/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk664(t1081: f64, t4180: f64, t1542: f64, t3001: f64, t1080: f64, t1054: f64, t1063: f64, t1073: f64, t1082: f64, t1531: f64, t1543: f64, t2925: f64, t2930: f64, t2955: f64, t2969: f64, t2974: f64, t2999: f64, t4062: f64, t4065: f64, t4067: f64, t4070: f64, t4107: f64, t4111: f64, t4117: f64, t4120: f64, t4125: f64, t4143: f64, t4147: f64, t4156: f64, t4158: f64, t4163: f64, t421: f64) -> (f64, f64, f64, f64) {
    let t4181 = t4180 * t1081;
    let t4184 = t1542 * t3001;
    let t4185 = t4184 * t1080;
    let t4188 = -0.310907e-1_f64 * t4117 * t421 + 1.0_f64 * t4120 * t1063 + 1.0_f64 * t2925 * t1531 - 2.0_f64 * t2930 * t4125 + 1.0_f64 * t1054 * t4143 + 0.32163958997385070134e2_f64 * t2955 * t4147 + t4062 - t4065 - t4067 + t4070 - t4107 - t4111 - 0.19751673498613801407e-1_f64 * t4156 + 0.5848223622634646207e0_f64 * t4158 * t1082 + 0.5848223622634646207e0_f64 * t2969 * t1543 - 0.11696447245269292414e1_f64 * t2974 * t4163 + 0.5848223622634646207e0_f64 * t1073 * t4181 + 0.17315859105681463759e2_f64 * t2999 * t4185;
    (t4181, t4184, t4185, t4188)
}
