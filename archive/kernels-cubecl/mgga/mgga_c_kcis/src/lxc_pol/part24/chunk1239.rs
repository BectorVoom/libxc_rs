//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1239/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1239<F: Float>(t1092: F, t1133: F, t69078: F, t7718: F, t27768: F, t95664: F, t28160: F, t28190: F, t100198: F, t100201: F, t100204: F, t100208: F, t27014: F, t29152: F, t8087: F, t8095: F, t96836: F, t97267: F, t97297: F) -> (F, F, F) {
    let t100212 = t1092 * t7718 * t69078 * t1133;
    let t100219 = t1092 * t95664 * t27768;
    let t100221 = t28190 * t28160;
    let t100224 = F::cast_from(0.92754700520833333334e-4_f64) * t97297 * t8087 + F::cast_from(0.11607361111111111111e-2_f64) * t100198 + F::cast_from(0.19345601851851851852e-2_f64) * t100201 - F::cast_from(0.23214722222222222222e-2_f64) * t100204 + F::cast_from(0.38691203703703703703e-3_f64) * t100208 + F::cast_from(0.51588271604938271604e-3_f64) * t100212 + F::cast_from(0.69505208333333333334e-3_f64) * t97267 * t8095 + F::cast_from(0.34752604166666666667e-3_f64) * t27014 * t29152 - F::cast_from(0.23214722222222222221e-2_f64) * t100219 + F::cast_from(0.23168402777777777778e-3_f64) * t100221 - F::cast_from(0.15445601851851851852e-3_f64) * t96836;
    (t100212, t100219, t100224)
}
