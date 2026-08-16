//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 406/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk406(t1344: f64, t1353: f64, t1356: f64, t219: f64, t654: f64, t679: f64, t726: f64, t734: f64, t739: f64, t1364: f64, t778: f64, t222: f64, t224: f64) -> (f64, f64, f64) {
    let t1373 = (t654 + t679 + t1344 + t1353 + t726 + t1356 - t734 - t739) * t219;
    let t1375 = t778 * t1364;
    let t1378 = -t1373 * t224 + 3.0_f64 * t1375 * t222;
    (t1373, t1375, t1378)
}
