//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1535/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1535(t11941: f64, t127: f64, t24032: f64, t371: f64, t15671: f64, t20016: f64, t1025: f64, t24022: f64, t1011: f64, t15993: f64, t23499: f64, t11875: f64, t11922: f64, t24012: f64) -> (f64, f64, f64, f64, f64) {
    let t79742 = t11941 * t371 * t127 * t24032;
    let t79744 = t15671 * t20016;
    let t79758 = t1025 * t371 * t127 * t24022;
    let t79811 = t1011 * t15993 * t23499;
    let t79818 = t11875 * t11922 * t24012;
    (t79742, t79744, t79758, t79811, t79818)
}
