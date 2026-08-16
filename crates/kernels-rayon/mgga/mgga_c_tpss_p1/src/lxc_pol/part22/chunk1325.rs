//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1325/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1325(t1206: f64, t19581: f64, t1338: f64, t2053: f64, t3537: f64, t623: f64, t2049: f64, t6076: f64, t77: f64, t1317: f64, t5506: f64, t19407: f64, t619: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65085 = t19581 * t1206;
    let t65094 = t2053 * t1338;
    let t65097 = t623 * t3537;
    let t65152 = t77 * t6076 * t2049;
    let t65157 = t5506 * t1317;
    let t65162 = t77 * t19407 * t619;
    (t65085, t65094, t65097, t65152, t65157, t65162)
}
