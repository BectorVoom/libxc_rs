//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1072/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1072(t121202: f64, t124: f64, t561: f64, t1353: f64, t9818: f64, t121174: f64, t49068: f64, t7301: f64, t119971: f64, t8705: f64, t121197: f64, t32244: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121203 = 0.74664478761315012733e-2_f64 * t121202;
    let t121204 = t124 * t561;
    let t121206 = t9818 * t121204 * t1353;
    let t121207 = t121174 * t121206;
    let t121210 = t7301 * t49068;
    let t121211 = t119971 * t8705 * t121210;
    let t121212 = 0.23511941766261123138e-4_f64 * t121211;
    let t121214 = 0.33852964522850660984e-1_f64 * t32244 * t121197;
    (t121203, t121204, t121206, t121207, t121210, t121212, t121214)
}
