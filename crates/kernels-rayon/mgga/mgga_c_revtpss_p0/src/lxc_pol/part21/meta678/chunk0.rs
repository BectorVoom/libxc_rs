//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2489/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2489(t13100: f64, t828: f64, t12699: f64, t3624: f64, t12772: f64, t12841: f64, t5340: f64, t12879: f64, t3625: f64, t3630: f64, t1260: f64, t12975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44225 = t828 * t13100;
    let t44230 = t12699 * t3624;
    let t44248 = t5340 * t12772 * t12841;
    let t44250 = t828 * t12879;
    let t44252 = t3625 * t44250 * t3630;
    let t44260 = t12975 * t1260;
    (t44225, t44230, t44248, t44250, t44252, t44260)
}
