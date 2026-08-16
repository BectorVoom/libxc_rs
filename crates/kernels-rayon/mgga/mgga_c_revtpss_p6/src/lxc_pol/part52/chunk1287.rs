//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1287/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1287(t1459: f64, t34366: f64, t2113: f64, t28265: f64, t5795: f64, t8731: f64, t28268: f64, t7334: f64, t8118: f64, t28280: f64, t1916: f64, t32779: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t129095 = 6.0_f64 * t1459 * t34366;
    let t129097 = 6.0_f64 * t2113 * t28265;
    let t129099 = 6.0_f64 * t5795 * t8731;
    let t129103 = 6.0_f64 * t2113 * t28268;
    let t129107 = 3.0_f64 * t8118 * t7334;
    let t129109 = 3.0_f64 * t2113 * t28280;
    let t129111 = 6.0_f64 * t1916 * t32779;
    (t129095, t129097, t129099, t129103, t129107, t129109, t129111)
}
