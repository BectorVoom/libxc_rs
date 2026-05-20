//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3147/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3147<F: Float>(t21233: F, t5381: F, t1774: F, t4186: F, t12787: F, t17448: F, t17729: F, t20797: F, t20959: F, t20963: F, t21022: F, t21028: F, t21119: F, t21228: F, t24739: F, t3720: F, t44551: F, t44952: F, t5046: F, t57100: F, t57382: F, t6640: F, t69783: F, t70639: F, t72011: F) -> (F, F) {
    let t82656 = t5381 * t21233;
    let t82664 = t1774 * t4186;
    let t82669 = F::cast_from(0.25724410870841842184e-2_f64) * t44551 * t3720 * t24739 * t21119 - F::cast_from(0.12862205435420921092e-2_f64) * t44952 * t3720 * t24739 * t21028 - F::cast_from(0.85748036236139473944e-3_f64) * t57100 * t6640 - F::cast_from(0.85748036236139473944e-3_f64) * t17448 * t21022 - F::cast_from(0.85748036236139473944e-3_f64) * t17448 * t21228 + F::cast_from(0.17149607247227894789e-2_f64) * t69783 + F::cast_from(0.47637797908966374413e-3_f64) * t82656 + F::cast_from(0.38586616306262763276e-2_f64) * t70639 * t20959 - F::cast_from(0.38586616306262763276e-2_f64) * t72011 * t20963 + F::cast_from(0.64311027177104605458e-3_f64) * t57382 * t20797 - F::cast_from(0.14291339372689912324e-2_f64) * t17729 * t12787 * t5046 * t82664;
    (t82664, t82669)
}
