//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 958/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk958(t644: f64, t77: f64, t7705: f64, t1497: f64, t1927: f64, t1926: f64, t1470: f64, t2247: f64, t1928: f64, t25099: f64, t25157: f64, t25162: f64, t25164: f64, t28116: f64, t28119: f64, t28127: f64, t28133: f64, t28138: f64, t28141: f64, t6958: f64, t6960: f64, t6963: f64, t6974: f64, t6978: f64, t7706: f64, t7709: f64, t7716: f64, t7720: f64) -> (f64, f64, f64, f64) {
    let t28147 = t77 * t7705 * t644;
    let t28150 = t1927 * t1497;
    let t28151 = t1926 * t28150;
    let t28154 = t2247 * t1470;
    let t28157 = t28116 * t1928 / 3.0_f64 + t28119 * t1928 / 3.0_f64 + t7709 * t6974 / 3.0_f64 + t7709 * t6978 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t28127 * t6960 + t6963 * t7716 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t6958 * t28133 + t6963 * t7720 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t28138 * t6960 + t28141 * t1928 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t25099 * t7706 - 5.0_f64 * t25157 * t28147 - 5.0_f64 / 3.0_f64 * t25162 * t28151 - 5.0_f64 / 3.0_f64 * t28154 * t25164;
    (t28147, t28150, t28154, t28157)
}
