//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk900;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk901;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta193(t1150: f64, t5104: f64, t1131: f64, t1732: f64, t3435: f64, t1149: f64, t3433: f64, t3358: f64, t3439: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t1160: f64, t1737: f64, t1168: f64, t1745: f64, t3415: f64, t3459: f64, t3466: f64, t5072: f64, t5080: f64, t5088: f64, t5090: f64, t5093: f64, t5096: f64, t5099: f64, t5102: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5105, t5107, t5108, t5109, t5111, t5117) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk900(t1150, t5104, t1131, t1732, t3435, t1149, t3433, t3358, t3439, t5044, t5049, t5054, t5058);
        let (t5120, t5125, t5142) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk901(t1160, t1737, t1168, t1745, t3358, t3415, t3459, t3466, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093, t5096, t5099, t5102);
    (t5105, t5107, t5108, t5109, t5111, t5117, t5120, t5125, t5142)
}
