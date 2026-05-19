//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1041/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1041<F: Float>(t1459: F, t1980: F, t33883: F, t7458: F, t1967: F, t8541: F, t31038: F, t527: F, t8497: F, t2001: F, t4528: F, t1998: F, t4523: F) -> (F, F, F, F, F, F) {
    let t34305 = t1980 * t7458 * t1459 * t33883;
    let t34307 = t1967 * t8541;
    let t34308 = F::cast_from(0.64311027177104605458e-2_f64) * t34307;
    let t34309 = t31038 * t527;
    let t34311 = t1967 * t8497;
    let t34312 = F::cast_from(0.25724410870841842184e-2_f64) * t34311;
    let t34313 = t2001 * t4528;
    let t34315 = t1998 * t4523;
    (t34305, t34308, t34309, t34312, t34313, t34315)
}
