//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2018/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2018(t103290: f64, t103291: f64, t103292: f64, t103293: f64, t103294: f64, t103296: f64, t103301: f64, t106058: f64, t106061: f64, t106063: f64, t106065: f64, t99035: f64) -> f64 {
    let t110414 = 0.17149607247227894789e-1_f64 * t106058 + t103290 - t103291 - t103292 - t103293 + t103294 + t103296 - 0.45351183609335988441e-1_f64 * t99035 + t103301 + 0.11433071498151929859e-3_f64 * t106061 + 0.40015750243531754507e-2_f64 * t106063 - 0.80031500487063509015e-2_f64 * t106065;
    t110414
}
