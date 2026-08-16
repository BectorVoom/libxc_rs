//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1519/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1519(t10246: f64, t2362: f64, t10236: f64, t108: f64, t101: f64, t10217: f64, t10229: f64, t10233: f64, t10237: f64, t10243: f64, t105: f64, t2344: f64, t2351: f64, t2354: f64, t656: f64, t659: f64, t97: f64) -> (f64, f64, f64, f64) {
    let t10247 = t10246 * t2362;
    let t10250 = -t10236;
    let t10251 = t108 * t10250;
    let t10254 = -440.0_f64 / 27.0_f64 * t10217 * t101 + 200.0_f64 / 9.0_f64 * t2344 * t659 - 50.0_f64 / 9.0_f64 * t656 * t2351 - 25.0_f64 / 3.0_f64 * t656 * t2354 - 10.0_f64 / 27.0_f64 * t97 * t10229 + 10.0_f64 / 3.0_f64 * t97 * t10233 + 5.0_f64 / 3.0_f64 * t97 * t10237 - 10.0_f64 / 27.0_f64 * t105 * t10243 + 10.0_f64 / 3.0_f64 * t105 * t10247 + 5.0_f64 / 3.0_f64 * t105 * t10251;
    (t10247, t10250, t10251, t10254)
}
