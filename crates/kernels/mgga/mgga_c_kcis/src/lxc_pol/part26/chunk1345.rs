//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1345/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1345<F: Float>(t22351: F, t27520: F, t28594: F, t28597: F, t102985: F, t102987: F, t102989: F, t102991: F, t102993: F, t102995: F, t102997: F, t102999: F, t103002: F) -> (F, F, F) {
    let t103004 = t27520 * t22351;
    let t103006 = t28594 * t28597;
    let t103008 = F::cast_from(0.89930555555555555557e-2_f64) * t102985 - F::cast_from(0.4046875e-1_f64) * t102987 - F::cast_from(0.28777777777777777779e0_f64) * t102989 + F::cast_from(0.59953703703703703705e-2_f64) * t102991 + F::cast_from(0.33333333333333333333e0_f64) * t102993 - F::cast_from(0.41666666666666666667e-1_f64) * t102995 + F::cast_from(0.68347222222222222224e0_f64) * t102997 - F::cast_from(0.20833333333333333333e-1_f64) * t102999 - F::cast_from(0.809375e-1_f64) * t103002 + F::cast_from(0.375e0_f64) * t103004 + F::cast_from(0.5e0_f64) * t103006;
    (t103004, t103006, t103008)
}
