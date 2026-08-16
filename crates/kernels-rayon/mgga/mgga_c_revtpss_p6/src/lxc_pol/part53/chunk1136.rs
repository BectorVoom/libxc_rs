//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1136/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1136(t28187: f64, t8568: f64, t33913: f64, t7239: f64, t33597: f64, t7235: f64, t32110: f64, t7732: f64, t121441: f64, t2014: f64, t7900: f64, t33667: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125500 = t8568 * t28187;
    let t125502 = t33913 * t7239;
    let t125505 = 3.0_f64 * t7235 * t33597;
    let t125507 = 2.0_f64 * t7732 * t32110;
    let t125510 = 3.0_f64 * t2014 * t121441 * t7900;
    let t125512 = 2.0_f64 * t7235 * t33667;
    (t125500, t125502, t125505, t125507, t125510, t125512)
}
