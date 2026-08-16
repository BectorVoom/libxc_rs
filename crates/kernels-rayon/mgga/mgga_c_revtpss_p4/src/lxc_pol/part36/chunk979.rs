//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 979/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk979(t1868: f64, t22486: f64, t5532: f64, t6836: f64, t1907: f64, t198: f64, t22483: f64, t22813: f64, t22925: f64, t22926: f64, t5536: f64, t5541: f64, t566: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64, t9588: f64) -> f64 {
    let t23068 = t22486 * t1868;
    let t23071 = t5532 * t6836;
    let t23077 = -3.0_f64 * t1907 * t22483 * t5541 + 6.0_f64 * t198 * t22813 * t566 + 18.0_f64 * t23068 * t5536 + 18.0_f64 * t23071 * t5536 - t22925 - t22926 + t9514 - t9517 - t9521 - t9524 + t9546 + t9569 - t9574 - t9577 - t9588;
    t23077
}
