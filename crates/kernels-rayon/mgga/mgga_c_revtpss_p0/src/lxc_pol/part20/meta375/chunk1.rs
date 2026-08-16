//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1359/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1359(t10878: f64, t2741: f64, t2722: f64, t853: f64, t10726: f64, t10786: f64, t2661: f64, t10943: f64, t2663: f64, t2645: f64, t2662: f64, t2749: f64) -> (f64, f64, f64, f64, f64) {
    let t40376 = t2741 * t10878;
    let t40378 = t853 * t2722;
    let t40381 = t2661 * t10726 * t40378 * t10786;
    let t40385 = t2661 * t10726 * t2663 * t10943;
    let t40390 = t2661 * t2662 * t853 * t2645 * t2749;
    (t40376, t40378, t40381, t40385, t40390)
}
