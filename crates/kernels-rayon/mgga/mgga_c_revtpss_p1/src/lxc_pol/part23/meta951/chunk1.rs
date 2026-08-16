//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3150/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3150(t12916: f64, t24752: f64, t3718: f64, t1261: f64, t12884: f64, t24232: f64, t247: f64, t17729: f64, t17753: f64, t20836: f64, t20903: f64, t20923: f64, t20956: f64, t21203: f64, t21246: f64, t24535: f64, t24834: f64, t3626: f64, t3647: f64, t3720: f64, t44551: f64, t5051: f64, t5274: f64, t59279: f64, t6421: f64, t69820: f64, t70303: f64, t82481: f64, t82664: f64) -> f64 {
    let t82749 = t3718 * t12916 * t24752;
    let t82757 = t1261 * t247 * t12884 * t24232;
    let t82763 = -0.17149607247227894789e-2_f64 * t69820 + 0.25724410870841842184e-2_f64 * t44551 * t3720 * t24834 * t59279 + 0.17149607247227894789e-2_f64 * t17729 * t3626 * t5051 * t82664 + 0.25724410870841842183e-2_f64 * t17729 * t3626 * t6421 * t82481 + 0.64311027177104605458e-3_f64 * t17753 * t3720 * t20956 * t20836 - 0.42874018118069736972e-3_f64 * t82749 - 0.14291339372689912324e-2_f64 * t70303 * t20923 + 0.64311027177104605458e-3_f64 * t5274 * t20903 + 0.95275595817932748827e-3_f64 * t82757 - 0.63517063878621832552e-3_f64 * t3647 * t24535 - 0.68598428988911579154e-2_f64 * t21203 * t21246;
    t82763
}
