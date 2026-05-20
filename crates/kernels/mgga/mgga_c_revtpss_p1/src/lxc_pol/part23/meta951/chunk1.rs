//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3150/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3150<F: Float>(t12916: F, t24752: F, t3718: F, t1261: F, t12884: F, t24232: F, t247: F, t17729: F, t17753: F, t20836: F, t20903: F, t20923: F, t20956: F, t21203: F, t21246: F, t24535: F, t24834: F, t3626: F, t3647: F, t3720: F, t44551: F, t5051: F, t5274: F, t59279: F, t6421: F, t69820: F, t70303: F, t82481: F, t82664: F) -> F {
    let t82749 = t3718 * t12916 * t24752;
    let t82757 = t1261 * t247 * t12884 * t24232;
    let t82763 = -F::cast_from(0.17149607247227894789e-2_f64) * t69820 + F::cast_from(0.25724410870841842184e-2_f64) * t44551 * t3720 * t24834 * t59279 + F::cast_from(0.17149607247227894789e-2_f64) * t17729 * t3626 * t5051 * t82664 + F::cast_from(0.25724410870841842183e-2_f64) * t17729 * t3626 * t6421 * t82481 + F::cast_from(0.64311027177104605458e-3_f64) * t17753 * t3720 * t20956 * t20836 - F::cast_from(0.42874018118069736972e-3_f64) * t82749 - F::cast_from(0.14291339372689912324e-2_f64) * t70303 * t20923 + F::cast_from(0.64311027177104605458e-3_f64) * t5274 * t20903 + F::cast_from(0.95275595817932748827e-3_f64) * t82757 - F::cast_from(0.63517063878621832552e-3_f64) * t3647 * t24535 - F::cast_from(0.68598428988911579154e-2_f64) * t21203 * t21246;
    t82763
}
