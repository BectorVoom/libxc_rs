//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1149/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1149(t136: f64, t1883: f64, t2457: f64, t10139: f64, t13926: f64, t543: f64, t4100: f64, t2782: f64, t10014: f64, t5741: f64, t13790: f64, t1398: f64) -> (f64, f64, f64, f64) {
    let t14219 = t1883 * t136;
    let t14220 = t14219 * t2457;
    let t14221 = t10139 * t14220;
    let t14224 = t13926 * t543;
    let t14225 = t4100 * t14224;
    let t14227 = 0.10975748638225852664e-1_f64 * t2782 * t14225;
    let t14229 = 0.19514881078765566038e-1_f64 * t10014 * t5741;
    let t14230 = t13790 * t1398;
    (t14221, t14227, t14229, t14230)
}
