//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1162/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1162(t10078: f64, t10082: f64, t10100: f64, t10104: f64, t10118: f64, t10131: f64, t10138: f64, t12970: f64, t12974: f64, t12978: f64, t12982: f64, t12986: f64, t12993: f64, t3271: f64) -> f64 {
    let t12994 = -119.0_f64 / 6912.0_f64 * t10078 - 7.0_f64 / 2304.0_f64 * t10082 + 7.0_f64 / 4608.0_f64 * t10100 + t3271 * t12970 / 384.0_f64 + t3271 * t12974 / 768.0_f64 + t3271 * t12978 / 768.0_f64 - t3271 * t12982 / 1536.0_f64 - t3271 * t12986 / 3072.0_f64 - t10104 - 7.0_f64 / 576.0_f64 * t10118 + 7.0_f64 / 144.0_f64 * t10131 - 7.0_f64 / 48.0_f64 * t10138 + t12993;
    t12994
}
