//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1157/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1157(t1181: f64, t2068: f64, t25727: f64, t604: f64, t31241: f64, t35436: f64, t35448: f64, t35452: f64, t35459: f64, t35469: f64, t37541: f64, t37559: f64, t37565: f64, t39962: f64, t39965: f64, t39967: f64, t39969: f64, t39971: f64, t39973: f64, t39977: f64) -> f64 {
    let t39981 = t2068 * t1181 * t604 * t25727;
    let t39983 = t37541 - 0.41930789719472202756e-3_f64 * t31241 - 0.80031500487063509015e-1_f64 * t35436 + t35448 - t35452 - 0.25724410870841842183e-2_f64 * t39962 + t37559 + t35459 - 0.51448821741683684367e-2_f64 * t35469 + 0.25724410870841842183e-2_f64 * t39965 + t37565 - 0.34299214494455789578e-2_f64 * t39967 + 0.17149607247227894789e-2_f64 * t39969 - 0.17149607247227894789e-2_f64 * t39971 + 0.85748036236139473944e-3_f64 * t39973 + 0.41930789719472202757e-3_f64 * t39977 + 0.42874018118069736972e-3_f64 * t39981;
    t39983
}
