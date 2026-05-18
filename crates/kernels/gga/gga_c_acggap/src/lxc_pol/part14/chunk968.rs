//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 968/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk968<F: Float>(t1967: F, t8541: F, t31038: F, t527: F, t8497: F, t1998: F, t4523: F, t7676: F, t8689: F, t1181: F, t5087: F, t604: F, t7426: F) -> (F, F, F, F, F, F) {
    let t34307 = t1967 * t8541;
    let t34308 = F::new(0.64311027177104605458e-2) * t34307;
    let t34309 = t31038 * t527;
    let t34311 = t1967 * t8497;
    let t34312 = F::new(0.25724410870841842184e-2) * t34311;
    let t34315 = t1998 * t4523;
    let t34317 = t7676 * t8689;
    let t34336 = t7426 * t1181 * t604 * t5087;
    (t34308, t34309, t34312, t34315, t34317, t34336)
}
