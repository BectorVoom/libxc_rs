//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 966/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk966(t4311: f64, t5999: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t23189: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64) -> (f64, f64) {
    let t23191 = 12.0_f64 * t4311 * t5999;
    let t23192 = -t10568 - t23189 + t9514 - t9517 - t9521 + t10577 + t10582 - t10584 - t10586 + t23191 - t9524;
    (t23191, t23192)
}
