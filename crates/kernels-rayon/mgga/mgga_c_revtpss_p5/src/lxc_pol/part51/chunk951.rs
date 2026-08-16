//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 951/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk951(t7315: f64, t8595: f64, t2014: f64, t531: f64, t8598: f64, t1353: f64, t4147: f64, t7316: f64, t8568: f64, t7239: f64, t1448: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32117 = t8595 * t7315;
    let t32118 = t2014 * t32117;
    let t32119 = t531 * t8598;
    let t32120 = t4147 * t1353;
    let t32121 = t32119 * t32120;
    let t32123 = 3.0_f64 * t2014 * t32121;
    let t32124 = t8568 * t7316;
    let t32126 = t8568 * t7239;
    let t32128 = t9593 * t1448;
    (t32117, t32118, t32119, t32121, t32123, t32124, t32126, t32128)
}
