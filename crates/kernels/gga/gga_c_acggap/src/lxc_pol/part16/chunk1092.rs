//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1092/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1092<F: Float>(t1165: F, t2068: F, t25727: F, t7351: F, t7337: F, t8480: F, t8774: F, t5727: F, t7647: F, t30321: F, t30325: F, t34240: F, t34263: F, t34273: F, t34284: F, t36993: F, t36998: F, t37003: F, t37009: F, t39112: F, t39114: F, t39118: F, t39122: F) -> F {
    let t39131 = t2068 * t1165 * t7351 * t25727;
    let t39134 = t7337 * t8480 * t8774;
    let t39136 = t7647 * t5727;
    let t39138 = F::cast_from(0.32155513588552302729e-2_f64) * t39112 + F::cast_from(0.32155513588552302729e-2_f64) * t39114 + F::cast_from(0.32155513588552302729e-2_f64) * t39118 + F::cast_from(0.21437009059034868486e-2_f64) * t39122 + t36993 - t34240 - t36998 - F::cast_from(0.12579236915841660827e-2_f64) * t34263 - t37003 - F::cast_from(0.80031500487063509015e-2_f64) * t34273 - F::cast_from(0.21437009059034868486e-3_f64) * t30321 - F::cast_from(0.80031500487063509015e-2_f64) * t34284 + t37009 + F::cast_from(0.94344276868812456204e-3_f64) * t30325 - F::cast_from(0.94344276868812456204e-3_f64) * t39131 + F::cast_from(0.10718504529517434243e-2_f64) * t39134 + F::cast_from(0.17149607247227894789e-2_f64) * t39136;
    t39138
}
