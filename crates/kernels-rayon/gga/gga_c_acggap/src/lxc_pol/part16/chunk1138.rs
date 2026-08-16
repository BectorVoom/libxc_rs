//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1138/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1138(t5698: f64, t7436: f64, t30924: f64, t30926: f64, t30928: f64, t30978: f64, t30982: f64, t30985: f64, t30987: f64, t30990: f64, t30993: f64, t35114: f64, t35146: f64, t35149: f64, t39720: f64, t39724: f64, t39733: f64, t39735: f64) -> f64 {
    let t39737 = t7436 * t5698;
    let t39742 = t35114 - 0.94344276868812456204e-2_f64 * t39720 + 0.10718504529517434243e-2_f64 * t39724 + 0.37737710747524982482e-2_f64 * t30924 - 0.56606566121287473723e-2_f64 * t30926 - 0.37737710747524982482e-2_f64 * t30928 - t35146 - t35149 + 0.80031500487063509016e-2_f64 * t30978 - 0.80031500487063509016e-2_f64 * t30982 - 0.22921875e-1_f64 * t39733 - t39735 / 8.0_f64 - t39737 / 12.0_f64 + 0.10718504529517434243e-2_f64 * t30985 - 0.12862205435420921092e-2_f64 * t30987 - t30990 - 0.95275595817932748827e-3_f64 * t30993;
    t39742
}
