//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3196/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3196<F: Float>(t21119: F, t82578: F, t21028: F, t1010: F, t22700: F, t1222: F, t1227: F, t12866: F, t17351: F, t17654: F, t17661: F, t17693: F, t17694: F, t17799: F, t20766: F, t20770: F, t20934: F, t21213: F, t21227: F, t5309: F, t5312: F, t57621: F, t57663: F, t71300: F, t81186: F, t82579: F, t83024: F) -> (F, F, F) {
    let t83943 = t82578 * t21119;
    let t83950 = t82578 * t21028;
    let t83962 = t22700 * t1010;
    let t83973 = F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t17661 * t21227 - F::cast_from(0.17149607247227894789e-2_f64) * t17654 * t17799 * t83943 - F::cast_from(0.25724410870841842183e-2_f64) * t17693 * t57621 * t83024 + F::cast_from(0.85748036236139473944e-3_f64) * t17351 * t17799 * t83950 + F::cast_from(0.85748036236139473944e-3_f64) * t57663 * t20934 - F::cast_from(0.85748036236139473944e-3_f64) * t17654 * t71300 * t20766 + F::cast_from(0.42874018118069736972e-3_f64) * t17351 * t71300 * t20770 + F::cast_from(77.0_f64) / F::cast_from(486.0_f64) * t83962 * t1227 - F::cast_from(0.7145669686344956162e-3_f64) * t12866 * t17694 * t82579 + t1222 * t5312 * t81186 / F::cast_from(72.0_f64) - F::cast_from(11.0_f64) / F::cast_from(54.0_f64) * t21213 * t5309;
    (t83943, t83950, t83973)
}
