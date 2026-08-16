//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1175/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1175(t32192: f64, t5629: f64, t8583: f64, t8589: f64, t121354: f64, t33969: f64, t8591: f64, t120991: f64, t121019: f64, t5676: f64, t121018: f64, t5674: f64, t94396: f64) -> (f64, f64, f64, f64) {
    let t125721 = t8583 * t8589 * t32192 * t5629;
    let t125732 = t8591 * t121354 * t33969;
    let t125749 = t120991 * t121019 * t5676;
    let t125753 = t121018 * t121019 * t5674 * t94396;
    (t125721, t125732, t125749, t125753)
}
