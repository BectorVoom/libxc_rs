//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 384/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk384(t1172: f64, t1175: f64, t1185: f64, t1187: f64, t1189: f64, t1192: f64, t1195: f64, t1196: f64, t1206: f64, t1268: f64, t1270: f64, t198: f64, t509: f64, t654: f64, t679: f64) -> f64 {
    let t1273 = t1268 * t1270 * t198 * t509 + 3.0_f64 * t1196 * t1206 * t198 + t1172 - t1175 + t1185 + t1187 + t1189 - t1192 - t1195 + t654 + t679;
    t1273
}
