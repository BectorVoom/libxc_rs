//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1138/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1138<F: Float>(t5698: F, t7436: F, t30924: F, t30926: F, t30928: F, t30978: F, t30982: F, t30985: F, t30987: F, t30990: F, t30993: F, t35114: F, t35146: F, t35149: F, t39720: F, t39724: F, t39733: F, t39735: F) -> F {
    let t39737 = t7436 * t5698;
    let t39742 = t35114 - F::cast_from(0.94344276868812456204e-2_f64) * t39720 + F::cast_from(0.10718504529517434243e-2_f64) * t39724 + F::cast_from(0.37737710747524982482e-2_f64) * t30924 - F::cast_from(0.56606566121287473723e-2_f64) * t30926 - F::cast_from(0.37737710747524982482e-2_f64) * t30928 - t35146 - t35149 + F::cast_from(0.80031500487063509016e-2_f64) * t30978 - F::cast_from(0.80031500487063509016e-2_f64) * t30982 - F::new(0.22921875e-1) * t39733 - t39735 / F::new(8.0) - t39737 / F::new(12.0) + F::cast_from(0.10718504529517434243e-2_f64) * t30985 - F::cast_from(0.12862205435420921092e-2_f64) * t30987 - t30990 - F::cast_from(0.95275595817932748827e-3_f64) * t30993;
    t39742
}
