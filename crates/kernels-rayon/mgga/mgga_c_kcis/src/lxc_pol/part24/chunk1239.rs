//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1239/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1239(t1092: f64, t1133: f64, t69078: f64, t7718: f64, t27768: f64, t95664: f64, t28160: f64, t28190: f64, t100198: f64, t100201: f64, t100204: f64, t100208: f64, t27014: f64, t29152: f64, t8087: f64, t8095: f64, t96836: f64, t97267: f64, t97297: f64) -> (f64, f64, f64) {
    let t100212 = t1092 * t7718 * t69078 * t1133;
    let t100219 = t1092 * t95664 * t27768;
    let t100221 = t28190 * t28160;
    let t100224 = 0.92754700520833333334e-4_f64 * t97297 * t8087 + 0.11607361111111111111e-2_f64 * t100198 + 0.19345601851851851852e-2_f64 * t100201 - 0.23214722222222222222e-2_f64 * t100204 + 0.38691203703703703703e-3_f64 * t100208 + 0.51588271604938271604e-3_f64 * t100212 + 0.69505208333333333334e-3_f64 * t97267 * t8095 + 0.34752604166666666667e-3_f64 * t27014 * t29152 - 0.23214722222222222221e-2_f64 * t100219 + 0.23168402777777777778e-3_f64 * t100221 - 0.15445601851851851852e-3_f64 * t96836;
    (t100212, t100219, t100224)
}
