//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1446/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1446(t11354: f64, t2881: f64, t2889: f64, t11315: f64, t11372: f64, t11358: f64, t11375: f64, t41316: f64, t41323: f64, t41353: f64, t41356: f64, t41359: f64, t41396: f64, t41402: f64, t41404: f64, t41406: f64, t41409: f64) -> (f64, f64, f64, f64, f64) {
    let t41412 = t11354 * t2881 * t2889;
    let t41414 = t11372 * t11315;
    let t41417 = t11358 * t2881 * t2889;
    let t41419 = t11375 * t11315;
    let t41421 = -0.108693e2_f64 * t41316 + 0.72462e1_f64 * t41323 - 0.20128333333333333334e1_f64 * t41353 + 0.24154e1_f64 * t41356 - 0.80513333333333333332e0_f64 * t41359 + 0.258925e1_f64 * t41396 - 0.485484375e1_f64 * t41402 - 0.3883875e1_f64 * t41404 + 0.22076e0_f64 * t41406 - 0.298026e1_f64 * t41409 + 0.11651625e2_f64 * t41412 - 0.51785e1_f64 * t41414 - 0.247573125e0_f64 * t41417 + 0.3300975e0_f64 * t41419;
    (t41412, t41414, t41417, t41419, t41421)
}
