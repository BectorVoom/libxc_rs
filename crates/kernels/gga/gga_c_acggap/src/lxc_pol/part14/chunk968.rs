//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 968/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk968<F: Float>(t7337: F, t8480: F, t8774: F, t5727: F, t7647: F, t30321: F, t30325: F, t34240: F, t34263: F, t34273: F, t34284: F, t36993: F, t36998: F, t37003: F, t37009: F, t39112: F, t39114: F, t39118: F, t39122: F, t39131: F) -> (F,) {
    let t39134 = t7337 * t8480 * t8774;
    let t39136 = t7647 * t5727;
    let t39138 = 0.32155513588552302729e-2 * t39112 + 0.32155513588552302729e-2 * t39114 + 0.32155513588552302729e-2 * t39118 + 0.21437009059034868486e-2 * t39122 + t36993 - t34240 - t36998 - 0.12579236915841660827e-2 * t34263 - t37003 - 0.80031500487063509015e-2 * t34273 - 0.21437009059034868486e-3 * t30321 - 0.80031500487063509015e-2 * t34284 + t37009 + 0.94344276868812456204e-3 * t30325 - 0.94344276868812456204e-3 * t39131 + 0.10718504529517434243e-2 * t39134 + 0.17149607247227894789e-2 * t39136;
    (t39138,)
}
