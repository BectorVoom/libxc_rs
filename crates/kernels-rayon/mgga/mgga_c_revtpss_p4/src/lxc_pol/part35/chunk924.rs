//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 924/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk924(t23400: f64, t23420: f64, t10566: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t1583: f64, t18865: f64, t1940: f64, t198: f64, t207: f64, t23186: f64, t23189: f64, t892: f64, t9514: f64, t9517: f64, t9521: f64) -> (f64, f64) {
    let t23421 = t23400 + t23420;
    let t23428 = t198 * t207 * t23421 * t892 - 3.0_f64 * t1583 * t18865 * t1940 + t10566 - t10568 + t10577 + t10582 - t10584 - t10586 - t23186 - t23189 + t9514 - t9517 - t9521;
    (t23421, t23428)
}
