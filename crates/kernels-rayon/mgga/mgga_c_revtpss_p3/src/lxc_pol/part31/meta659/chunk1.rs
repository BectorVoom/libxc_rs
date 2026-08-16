//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2231/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2231(t108986: f64, t1926: f64, t2247: f64, t5826: f64, t60673: f64, t6957: f64, t101222: f64, t101230: f64, t101333: f64, t10309: f64, t108966: f64, t108971: f64, t108975: f64, t108979: f64, t108983: f64, t25157: f64, t25162: f64, t25164: f64, t28147: f64, t28151: f64, t28154: f64, t34176: f64, t6960: f64) -> f64 {
    let t108987 = t1926 * t108986;
    let t108990 = t2247 * t5826;
    let t108995 = t60673 * t6957;
    let t109001 = -10.0_f64 / 3.0_f64 * t101230 * t28151 - 10.0_f64 / 3.0_f64 * t108966 * t25164 - 10.0_f64 * t101333 * t28147 - 10.0_f64 / 3.0_f64 * t25162 * t108971 - 10.0_f64 * t25157 * t108975 - 10.0_f64 / 3.0_f64 * t25162 * t108979 - 5.0_f64 * t25157 * t108983 - 5.0_f64 / 3.0_f64 * t25162 * t108987 - 5.0_f64 / 3.0_f64 * t108990 * t25164 - 10.0_f64 / 3.0_f64 * t28154 * t101222 + 5.0_f64 / 6.0_f64 * t108995 * t6960 + 20.0_f64 * t10309 * t34176 * t28147;
    t109001
}
