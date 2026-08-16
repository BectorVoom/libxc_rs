//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1097/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1097(t1181: f64, t604: f64, t6192: f64, t7426: f64, t30330: f64, t30334: f64, t30340: f64, t30343: f64, t30347: f64, t34309: f64, t34312: f64, t34336: f64, t34341: f64, t34348: f64, t37021: f64, t37022: f64, t39182: f64, t39186: f64, t39189: f64, t39192: f64, t39194: f64) -> f64 {
    let t39203 = t7426 * t1181 * t604 * t6192;
    let t39205 = -0.10718504529517434243e-2_f64 * t39182 - 0.10718504529517434243e-2_f64 * t39186 - 0.7145669686344956162e-3_f64 * t39189 + 0.80031500487063509015e-2_f64 * t34309 + t34312 + t37021 + t37022 + 0.17149607247227894789e-1_f64 * t39192 - 0.68598428988911579156e-2_f64 * t39194 - 0.10718504529517434243e-2_f64 * t30330 - 0.42874018118069736972e-3_f64 * t30334 + t30340 + 0.62896184579208304136e-3_f64 * t34336 + 0.53592522647587171215e-3_f64 * t30343 + 0.21437009059034868486e-3_f64 * t30347 + t34341 + 0.31448092289604152068e-3_f64 * t39203 - t34348;
    t39205
}
