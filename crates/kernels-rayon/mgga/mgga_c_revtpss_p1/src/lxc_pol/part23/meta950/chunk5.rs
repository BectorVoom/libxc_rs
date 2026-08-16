//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3147/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3147(t21233: f64, t5381: f64, t1774: f64, t4186: f64, t12787: f64, t17448: f64, t17729: f64, t20797: f64, t20959: f64, t20963: f64, t21022: f64, t21028: f64, t21119: f64, t21228: f64, t24739: f64, t3720: f64, t44551: f64, t44952: f64, t5046: f64, t57100: f64, t57382: f64, t6640: f64, t69783: f64, t70639: f64, t72011: f64) -> (f64, f64) {
    let t82656 = t5381 * t21233;
    let t82664 = t1774 * t4186;
    let t82669 = 0.25724410870841842184e-2_f64 * t44551 * t3720 * t24739 * t21119 - 0.12862205435420921092e-2_f64 * t44952 * t3720 * t24739 * t21028 - 0.85748036236139473944e-3_f64 * t57100 * t6640 - 0.85748036236139473944e-3_f64 * t17448 * t21022 - 0.85748036236139473944e-3_f64 * t17448 * t21228 + 0.17149607247227894789e-2_f64 * t69783 + 0.47637797908966374413e-3_f64 * t82656 + 0.38586616306262763276e-2_f64 * t70639 * t20959 - 0.38586616306262763276e-2_f64 * t72011 * t20963 + 0.64311027177104605458e-3_f64 * t57382 * t20797 - 0.14291339372689912324e-2_f64 * t17729 * t12787 * t5046 * t82664;
    (t82664, t82669)
}
