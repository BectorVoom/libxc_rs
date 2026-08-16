//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 999/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk999(t10681: f64, t10682: f64, t10703: f64, t10732: f64, t219: f64, t73: f64, t776: f64, t1364: f64, t2387: f64, t2116: f64, t3610: f64, t799: f64) -> (f64, f64, f64, f64) {
    let t10735 = (t10681 + t10682 + t10703 + t10732) * t219;
    let t10745 = t776 * t73;
    let t10750 = t2387 * t1364;
    let t10751 = t10750 * t2116;
    let t10754 = t799 * t3610;
    (t10735, t10745, t10751, t10754)
}
