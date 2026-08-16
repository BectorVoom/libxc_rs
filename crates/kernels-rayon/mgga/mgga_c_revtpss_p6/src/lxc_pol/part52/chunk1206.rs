//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1206/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1206(t34053: f64, t686: f64, t72: f64, t32474: f64, t122034: f64, t27341: f64, t119915: f64, t119937: f64, t121902: f64, t121914: f64, t121921: f64, t126214: f64, t126226: f64, t126232: f64, t127704: f64, t3140: f64, t4469: f64, t7073: f64, t8477: f64, t8652: f64) -> (f64, f64) {
    let t127724 = t34053 * t72 * t686;
    let t127725 = t32474 * t127724;
    let t127727 = t122034 * t27341;
    let t127730 = t119915 + 0.57119737665102352616e0_f64 * t8477 * t4469 * t3140 * t8652 + 0.7437465841810202164e-3_f64 * t126214 - 0.50779446784275991476e-1_f64 * t121902 - 0.34708173928447610099e-2_f64 * t126226 + 0.225875734067843736e-2_f64 * t126232 - 0.14279934416275588154e-1_f64 * t121914 + t119937 + 0.17347256376410398924e1_f64 * t127704 * t7073 + 0.25389723392137995738e-1_f64 * t127725 - 0.28912093960683998207e-1_f64 * t127727 - 0.14279934416275588154e-1_f64 * t121921;
    (t127724, t127730)
}
