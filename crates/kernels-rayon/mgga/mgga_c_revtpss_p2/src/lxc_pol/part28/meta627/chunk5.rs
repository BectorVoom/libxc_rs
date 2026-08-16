//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2252/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2252(t1497: f64, t6977: f64, t1926: f64, t1927: f64, t4241: f64, t25163: f64, t7715: f64, t101187: f64, t101190: f64, t101193: f64, t101200: f64, t101204: f64, t101211: f64, t10309: f64, t1928: f64, t25157: f64, t25162: f64, t28147: f64, t28151: f64, t32592: f64, t92565: f64, t92588: f64) -> f64 {
    let t101214 = t6977 * t1497;
    let t101215 = t1926 * t101214;
    let t101218 = t1927 * t4241;
    let t101219 = t1926 * t101218;
    let t101222 = t7715 * t25163;
    let t101225 = t101187 * t1928 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t101190 * t1928 + 2.0_f64 / 3.0_f64 * t101193 * t1928 + 20.0_f64 * t10309 * t32592 * t28147 - 10.0_f64 * t25157 * t101200 - 5.0_f64 * t25157 * t101204 - 10.0_f64 / 3.0_f64 * t92565 * t28151 - 5.0_f64 / 3.0_f64 * t92588 * t28151 - 10.0_f64 / 3.0_f64 * t25162 * t101211 - 10.0_f64 / 3.0_f64 * t25162 * t101215 - 10.0_f64 / 3.0_f64 * t25162 * t101219 - 10.0_f64 / 3.0_f64 * t25162 * t101222;
    t101225
}
