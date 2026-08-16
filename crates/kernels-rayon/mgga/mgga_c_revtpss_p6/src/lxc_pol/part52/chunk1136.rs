//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1136/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1136(t124: f64, t561: f64, t1353: f64, t9818: f64, t121174: f64, t49068: f64, t7301: f64, t119971: f64, t8705: f64, t32265: f64, t3974: f64, t119967: f64, t121173: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121204 = t124 * t561;
    let t121206 = t9818 * t121204 * t1353;
    let t121207 = t121174 * t121206;
    let t121208 = 0.26773803678175077508e-3_f64 * t121207;
    let t121210 = t7301 * t49068;
    let t121211 = t119971 * t8705 * t121210;
    let t121227 = t32265 * t3974;
    let t121232 = t119967 * t121173;
    (t121204, t121206, t121208, t121210, t121211, t121227, t121232)
}
