//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1136/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1136(t1405: f64, t32272: f64, t32269: f64, t3974: f64, t120981: f64, t120986: f64, t32710: f64, t1389: f64, t31752: f64, t32192: f64, t32282: f64, t8583: f64, t8584: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120994 = t32272 * t1405;
    let t120995 = 0.17354086964223805049e-2_f64 * t120994;
    let t120996 = t32269 * t3974;
    let t121000 = t32269 * t120981;
    let t121003 = t32710 * t120986;
    let t121004 = 0.13223814266738539448e-3_f64 * t121003;
    let t121011 = t31752 * t32192 * t1389;
    let t121018 = t8583 * t8584 * t32282;
    (t120995, t120996, t121000, t121004, t121011, t121018)
}
