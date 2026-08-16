//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1005/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1005(t10621: f64, t10656: f64, t10772: f64, t10816: f64, t219: f64, t3693: f64, t1395: f64, t2407: f64, t8348: f64, t3721: f64, t818: f64, t2406: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t10818 = t10621 + t10656 + t10772 + t10816;
    let t10819 = param_beta * t10818;
    let t10821 = t3693 * t219;
    let t10833 = t8348 * t1395 * t2407;
    let t10836 = t3721 * t818;
    let t10837 = t2406 * t10836;
    (t10818, t10819, t10821, t10833, t10837)
}
