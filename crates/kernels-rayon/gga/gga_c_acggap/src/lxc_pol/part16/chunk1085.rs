//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1085/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1085(t7315: f64, t9622: f64, t2016: f64, t9626: f64, t30280: f64, t34082: f64, t34092: f64, t34095: f64, t34100: f64, t34102: f64, t34107: f64, t34131: f64, t34133: f64, t36950: f64, t36961: f64, t39026: f64, t39029: f64, t39031: f64, t39035: f64, t39039: f64) -> f64 {
    let t39041 = t7315 * t9622;
    let t39043 = t2016 * t9626;
    let t39047 = 0.14291339372689912324e-3_f64 * t30280 - t34082 - 0.10718504529517434243e-3_f64 * t39026 - 0.10718504529517434243e-3_f64 * t39029 + 0.94344276868812456204e-2_f64 * t39031 + 0.42874018118069736972e-3_f64 * t39035 - 0.15724046144802076034e-2_f64 * t39039 + 11.0_f64 / 384.0_f64 * t39041 + 11.0_f64 / 1152.0_f64 * t39043 + t34092 - 0.62896184579208304135e-3_f64 * t34095 - t34100 + t34102 + 0.94344276868812456205e-2_f64 * t34107 + t36950 + t34131 - t34133 - t36961;
    t39047
}
