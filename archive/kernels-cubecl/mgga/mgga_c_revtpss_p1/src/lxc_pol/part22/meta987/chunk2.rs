//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3349/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3349<F: Float>(t2857: F, t60717: F, t141: F, t930: F, t41361: F, t41363: F, t41610: F, t51967: F, t51973: F, t51978: F, t63299: F, t63304: F, t63308: F, t63311: F, t63315: F, t63320: F, t63325: F, t63328: F) -> (F, F, F) {
    let t63330 = t2857 * t60717;
    let t63332 = t141 * t930 * t63330;
    let t63334 = F::cast_from(0.59793333333333333334e0_f64) * t63299 + F::cast_from(0.39862222222222222223e1_f64) * t63304 - F::cast_from(0.71752000000000000002e1_f64) * t63308 + t41610 - F::cast_from(0.98587999999999999998e0_f64) * t63311 + F::cast_from(0.197176e1_f64) * t63315 + F::cast_from(0.19931111111111111111e0_f64) * t51967 - F::cast_from(0.5314962962962962963e0_f64) * t51973 + F::cast_from(0.62007901234567901235e0_f64) * t51978 + F::cast_from(0.10954222222222222222e0_f64) * t63320 + F::cast_from(0.62007901234567901237e0_f64) * t41361 + F::cast_from(0.26574814814814814816e0_f64) * t41363 - F::cast_from(0.13287407407407407407e1_f64) * t63325 + F::cast_from(0.47834666666666666668e1_f64) * t63328 + F::cast_from(0.32862666666666666666e0_f64) * t63332;
    (t63330, t63332, t63334)
}
