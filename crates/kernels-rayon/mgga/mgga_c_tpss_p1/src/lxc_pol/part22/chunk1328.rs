//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1328/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1328(t1317: f64, t2045: f64, t77: f64, t3486: f64, t615: f64, t10440: f64, t84: f64, t1290: f64, t7679: f64, t1976: f64, t3426: f64, t3432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65285 = t77 * t2045 * t1317;
    let t65289 = t77 * t615 * t3486;
    let t65293 = t77 * t84 * t10440;
    let t65296 = t7679 * t1290;
    let t65299 = t1976 * t3426;
    let t65302 = t1976 * t3432;
    (t65285, t65289, t65293, t65296, t65299, t65302)
}
