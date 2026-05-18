//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1353/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1353<F: Float>(t27014: F, t28160: F, t15573: F, t28210: F, t7788: F, t15255: F, t92693: F, t96736: F, t26955: F, t26966: F, t27090: F, t28190: F, t28211: F, t8091: F, t92590: F, t92814: F, t92816: F, t92818: F, t92820: F, t92822: F, t93050: F, t96720: F) -> (F, F) {
    let t96952 = F::new(0.23168402777777777778e-3) * t27014 * t28160;
    let t96957 = F::new(0.23168402777777777778e-3) * t7788 * t15573 * t28210;
    let t96968 = t92693 * t96736 * t15255;
    let t96971 = F::new(0.34752604166666666667e-3) * t28190 * t27090 + t96952 - F::new(0.18534722222222222222e-2) * t26966 * t28211 + t96957 + F::new(0.11584201388888888889e-3) * t92814 + F::new(0.30918233506944444444e-4) * t92816 + F::new(0.77382407407407407407e-3) * t92818 + F::new(0.12897067901234567901e-2) * t92820 - F::new(0.61782407407407407408e-3) * t92822 + F::new(0.24777891269883300782e-5) * t93050 * t96720 - F::new(0.11584201388888888889e-3) * t92590 * t8091 - F::new(0.30918233506944444444e-4) * t26955 * t96968;
    (t96968, t96971)
}
