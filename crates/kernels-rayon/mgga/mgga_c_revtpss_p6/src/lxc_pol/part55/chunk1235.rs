//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1235/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1235(t28167: f64, t38099: f64, t5627: f64, t109269: f64, t32578: f64, t27833: f64, t8718: f64, t32626: f64, t7901: f64, t34328: f64, t7235: f64, t651: f64, t7002: f64, t8065: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128277 = 6.0_f64 * t28167 * t38099 * t5627;
    let t128279 = 2.0_f64 * t109269 * t32578;
    let t128280 = t27833 * t8718;
    let t128282 = 3.0_f64 * t32626 * t7901;
    let t128284 = t7235 * t34328;
    let t128287 = 2.0_f64 * t651 * t8065 * t7002;
    (t128277, t128279, t128280, t128282, t128284, t128287)
}
