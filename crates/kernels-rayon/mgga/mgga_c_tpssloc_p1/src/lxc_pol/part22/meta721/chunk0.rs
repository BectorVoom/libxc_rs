//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2344/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2344(t20908: f64, t2697: f64, t1509: f64, t5611: f64, t13222: f64, t13251: f64, t16914: f64, t16924: f64, t17009: f64, t20896: f64, t2623: f64, t2643: f64, t2647: f64, t46692: f64, t47044: f64, t47047: f64, t5593: f64, t58859: f64, t58873: f64, t58885: f64, t58890: f64, t58900: f64, t829: f64) -> f64 {
    let t68021 = t2697 * t20908;
    let t68025 = t5611 * t1509;
    let t68048 = 7.0_f64 / 1152.0_f64 * t68021 - 5.0_f64 / 128.0_f64 * t2623 * t20896 - t2643 * t46692 * t68025 * t829 / 1024.0_f64 + t2643 * t13222 * t68025 * t2647 / 256.0_f64 + 7.0_f64 / 1536.0_f64 * t58859 - 7.0_f64 / 192.0_f64 * t58873 - t13251 * t17009 / 512.0_f64 - 7.0_f64 / 384.0_f64 * t58885 + 7.0_f64 / 1536.0_f64 * t58890 - 7.0_f64 / 256.0_f64 * t58900 - 595.0_f64 / 3456.0_f64 * t47047 + t47044 * t5593 / 128.0_f64 + t13251 * t16924 / 128.0_f64 + t13251 * t16914 / 128.0_f64;
    t68048
}
