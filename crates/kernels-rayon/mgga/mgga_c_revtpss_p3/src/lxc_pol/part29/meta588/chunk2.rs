//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1943/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1943(t670: f64, t7373: f64, t101451: f64, t101453: f64, t101455: f64, t101458: f64, t101461: f64, t101464: f64, t101466: f64, t94976: f64, t94979: f64, t94981: f64, t95397: f64) -> (f64, f64) {
    let t101725 = t670 * t7373;
    let t101754 = 22.0_f64 / 9.0_f64 * t101451;
    let t101755 = 8.0_f64 / 3.0_f64 * t101453;
    let t101756 = 4.0_f64 / 3.0_f64 * t101455;
    let t101760 = -t95397 - 44.0_f64 / 9.0_f64 * t94976 - 4.0_f64 / 3.0_f64 * t94979 + 2.0_f64 / 3.0_f64 * t94981 - t101754 - t101755 + t101756 - 3.0_f64 / 2.0_f64 * t101458 + t101461 + t101464 / 2.0_f64 - t101466 / 4.0_f64;
    (t101725, t101760)
}
