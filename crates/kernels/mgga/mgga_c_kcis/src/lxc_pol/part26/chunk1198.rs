//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1198/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1198<F: Float>(t16622: F, t27543: F, t6012: F, t22351: F, t27520: F, t28594: F, t28597: F, t102985: F, t102987: F, t102989: F, t102991: F, t102993: F, t102995: F, t102997: F, t102999: F, t3734: F, t7296: F) -> (F, F, F, F, F) {
    let t103002 = t16622 * t27543 * t6012;
    let t103004 = t27520 * t22351;
    let t103006 = t28594 * t28597;
    let t103008 = 0.89930555555555555557e-2 * t102985 - 0.4046875e-1 * t102987 - 0.28777777777777777779e0 * t102989 + 0.59953703703703703705e-2 * t102991 + 0.33333333333333333333e0 * t102993 - 0.41666666666666666667e-1 * t102995 + 0.68347222222222222224e0 * t102997 - 0.20833333333333333333e-1 * t102999 - 0.809375e-1 * t103002 + 0.375e0 * t103004 + 0.5e0 * t103006;
    let t103010 = t3734 * t7296;
    (t103002, t103004, t103006, t103008, t103010)
}
