//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1092/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1092(t1165: f64, t2068: f64, t25727: f64, t7351: f64, t7337: f64, t8480: f64, t8774: f64, t5727: f64, t7647: f64, t30321: f64, t30325: f64, t34240: f64, t34263: f64, t34273: f64, t34284: f64, t36993: f64, t36998: f64, t37003: f64, t37009: f64, t39112: f64, t39114: f64, t39118: f64, t39122: f64) -> f64 {
    let t39131 = t2068 * t1165 * t7351 * t25727;
    let t39134 = t7337 * t8480 * t8774;
    let t39136 = t7647 * t5727;
    let t39138 = 0.32155513588552302729e-2_f64 * t39112 + 0.32155513588552302729e-2_f64 * t39114 + 0.32155513588552302729e-2_f64 * t39118 + 0.21437009059034868486e-2_f64 * t39122 + t36993 - t34240 - t36998 - 0.12579236915841660827e-2_f64 * t34263 - t37003 - 0.80031500487063509015e-2_f64 * t34273 - 0.21437009059034868486e-3_f64 * t30321 - 0.80031500487063509015e-2_f64 * t34284 + t37009 + 0.94344276868812456204e-3_f64 * t30325 - 0.94344276868812456204e-3_f64 * t39131 + 0.10718504529517434243e-2_f64 * t39134 + 0.17149607247227894789e-2_f64 * t39136;
    t39138
}
