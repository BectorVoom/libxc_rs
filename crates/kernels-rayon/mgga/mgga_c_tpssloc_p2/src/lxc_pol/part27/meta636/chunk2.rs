//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2146/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2146(t23040: f64, t4166: f64, t831: f64, t81808: f64, t4191: f64, t81749: f64, t4240: f64, t13248: f64, t25084: f64, t13326: f64, t23146: f64, t13210: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87261 = t4166 * t23040;
    let t87262 = t87261 * t831;
    let t87263 = 7.0_f64 / 1152.0_f64 * t87262;
    let t87268 = 119.0_f64 / 3456.0_f64 * t81808;
    let t87270 = t81749 * t4191;
    let t87271 = 7.0_f64 / 288.0_f64 * t87270;
    let t87272 = t81749 * t4240;
    let t87273 = 7.0_f64 / 1152.0_f64 * t87272;
    let t87274 = t25084 * t13248;
    let t87276 = t23146 * t13326;
    let t87278 = t23146 * t13210;
    (t87263, t87268, t87271, t87273, t87274, t87276, t87278)
}
