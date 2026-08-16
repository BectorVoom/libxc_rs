//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1383/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1383(t14224: f64, t4100: f64, t2782: f64, t10014: f64, t5741: f64, t13790: f64, t1398: f64, t10022: f64, t1892: f64, t4086: f64, t786: f64, t4104: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14225 = t4100 * t14224;
    let t14227 = 0.10975748638225852664e-1_f64 * t2782 * t14225;
    let t14229 = 0.19514881078765566038e-1_f64 * t10014 * t5741;
    let t14230 = t13790 * t1398;
    let t14231 = t10022 * t14230;
    let t14233 = 0.21951497276451705328e-1_f64 * t2782 * t14231;
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14241 = 0.19514881078765566038e-1_f64 * t14239 * t4104;
    (t14227, t14229, t14230, t14233, t14239, t14241)
}
