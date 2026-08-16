//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3873/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3873(t3889: f64, t3944: f64, t48825: f64, t48827: f64, t48829: f64, t48833: f64, t48837: f64, t48845: f64, t48847: f64, t48849: f64, t48851: f64, t48853: f64, t6883: f64, t800: f64) -> f64 {
    let t74458 = -0.10164000561857065645e-2_f64 * t48825 + 0.60976381323476959249e-2_f64 * t48827 + 0.22589491248727328396e-6_f64 * t48829 + t3944 * t800 * t6883 * t3889 / 16.0_f64 + 0.57800528129545867621e-2_f64 * t48833 + 0.12004725073059526352e-1_f64 * t48837 + 0.17149607247227894789e-2_f64 * t48845 - 0.30488190661738479624e-3_f64 * t48847 - 0.10276933901433255264e-1_f64 * t48849 - 0.90702367218671976884e-1_f64 * t48851 + 0.14450132032386466905e-2_f64 * t48853;
    t74458
}
