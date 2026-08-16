//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 714/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk714(t12706: f64, t10628: f64, t2365: f64, t6111: f64, t10893: f64, t959: f64, t13079: f64, t13098: f64, t13102: f64, t13106: f64, t13110: f64, t13113: f64, t13114: f64, t13115: f64, t13116: f64, t317: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64) {
    let t13117 = 0.63904876589867916127e-1_f64 * t12706;
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13120 = 0.59584149919750711116e-1_f64 * t13119;
    let t13121 = t10893 * t959;
    let t13123 = t13079 + 0.35750489951850426669e0_f64 * t13098 * t317 - 0.35750489951850426669e0_f64 * t797 * t13102 - 0.23005755572352449806e1_f64 * t813 * t13106 + 0.23005755572352449806e1_f64 * t833 * t13110 - t13113 - t13114 + t13115 + t13116 + t13117 + t13120 + 0.29792074959875355558e-1_f64 * t13121;
    (t13118, t13123)
}
