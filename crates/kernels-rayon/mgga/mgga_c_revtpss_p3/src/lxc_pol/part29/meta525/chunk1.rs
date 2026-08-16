//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1853/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1853(t25304: f64, t25949: f64, t1419: f64, t7063: f64, t25898: f64, t1955: f64, t7282: f64, t9656: f64, t281: f64, t555: f64, t93238: f64, t1426: f64, t94609: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94776 = t25304 * t25949;
    let t94801 = t7063 * t1419;
    let t94802 = t94801 * t25898;
    let t94823 = t1955 * t7282 * t9656;
    let t94849 = t281 * t93238 * t555;
    let t94878 = t94609 * t1426;
    (t94776, t94801, t94802, t94823, t94849, t94878)
}
