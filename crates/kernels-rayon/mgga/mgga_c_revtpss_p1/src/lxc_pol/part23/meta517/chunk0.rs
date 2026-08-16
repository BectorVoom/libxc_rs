//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2024/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2024(t1214: f64, t5825: f64, t5296: f64, t1042: f64, t3172: f64, t6630: f64, t3600: f64, t247: f64, t3634: f64, t6425: f64, t1261: f64, t1238: f64, t12882: f64, t12893: f64, t12900: f64, t12905: f64, t12985: f64, t17509: f64, t17546: f64, t17556: f64, t21177: f64, t3711: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21182 = t5825 * t1214;
    let t21183 = t5296 * t21182;
    let t21184 = t1042 * t21183;
    let t21188 = t3172 * t6630;
    let t21189 = t3600 * t21188;
    let t21192 = t247 * t3634 * t6425;
    let t21193 = t1261 * t21192;
    let t21196 = -0.72409452821628889107e-2_f64 * t21177 * t1238 + 0.31758531939310916275e-4_f64 * t12882 - 0.47637797908966374413e-4_f64 * t12893 + t12900 + 0.14291339372689912324e-3_f64 * t3711 * t21184 - 0.47637797908966374413e-4_f64 * t12905 + 0.28582678745379824648e-3_f64 * t21189 - t17509 - 0.19055119163586549765e-3_f64 * t21193 + t17546 + t17556 + 0.47637797908966374413e-4_f64 * t12985;
    (t21182, t21183, t21184, t21188, t21189, t21192, t21193, t21196)
}
