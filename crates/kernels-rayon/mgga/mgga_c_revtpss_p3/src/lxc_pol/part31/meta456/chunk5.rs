//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1654/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1654(t1261: f64, t21192: f64, t1238: f64, t12882: f64, t12893: f64, t12900: f64, t12905: f64, t12985: f64, t17509: f64, t17546: f64, t17556: f64, t21177: f64, t21184: f64, t21189: f64, t3711: f64) -> f64 {
    let t21193 = t1261 * t21192;
    let t21196 = -0.72409452821628889107e-2_f64 * t21177 * t1238 + 0.31758531939310916275e-4_f64 * t12882 - 0.47637797908966374413e-4_f64 * t12893 + t12900 + 0.14291339372689912324e-3_f64 * t3711 * t21184 - 0.47637797908966374413e-4_f64 * t12905 + 0.28582678745379824648e-3_f64 * t21189 - t17509 - 0.19055119163586549765e-3_f64 * t21193 + t17546 + t17556 + 0.47637797908966374413e-4_f64 * t12985;
    t21196
}
