//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1140/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1140(t2118: f64, t6071: f64, t1967: f64, t9724: f64, t31002: f64, t31016: f64, t31021: f64, t31023: f64, t35176: f64, t35184: f64, t35186: f64, t35191: f64, t35195: f64, t35199: f64, t37426: f64, t37435: f64, t39746: f64, t39750: f64, t39756: f64, t39763: f64) -> f64 {
    let t39765 = t2118 * t6071;
    let t39767 = t1967 * t9724;
    let t39769 = t31002 + t31016 - t31021 + t31023 + 0.10718504529517434243e-3_f64 * t39746 + 0.10718504529517434243e-3_f64 * t39750 - 0.41930789719472202756e-3_f64 * t35176 + 0.53592522647587171215e-3_f64 * t39756 + t37426 - 0.41930789719472202757e-3_f64 * t35184 - 0.6431102717710460546e-2_f64 * t35186 + t35191 - t35195 + t35199 - 0.94344276868812456204e-2_f64 * t39763 - 0.42874018118069736972e-3_f64 * t39765 - 0.28303283060643736861e-2_f64 * t39767 - t37435;
    t39769
}
