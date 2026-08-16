//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1258/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1258(t122278: f64, t27873: f64, t122274: f64, t125663: f64, t121029: f64, t121044: f64, t122351: f64, t122355: f64, t122443: f64, t125659: f64, t32674: f64, t32678: f64, t34231: f64, t7926: f64) -> f64 {
    let t128665 = t122278 * t27873;
    let t128673 = t122274 * t27873;
    let t128676 = 0.150583822711895824e-3_f64 * t125663;
    let t128677 = -t122351 - 0.28912093960683998207e-1_f64 * t128665 + 0.57119737665102352616e0_f64 * t34231 * t32674 + 0.57119737665102352616e0_f64 * t34231 * t32678 + t121029 + 0.8673628188205199462e0_f64 * t122443 * t7926 + 0.51405703062096148813e-1_f64 * t128673 - 0.225875734067843736e-2_f64 * t125659 + t128676 - t121044 - t122355;
    t128677
}
