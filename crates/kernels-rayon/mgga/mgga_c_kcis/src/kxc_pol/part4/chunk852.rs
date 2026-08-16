//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 852/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk852(t1102: f64, t1360: f64, t1924: f64, t344: f64, t486: f64, t5454: f64, t5460: f64, t5465: f64, t5483: f64, t5486: f64, t5490: f64, t5495: f64, t5500: f64, t5528: f64, t5623: f64) -> f64 {
    let t5626 = 0.98556445e-3_f64 * t1102 * t5454 + 0.7391733375e-3_f64 * t1102 * t5460 - 0.1478346675e-2_f64 * t1102 * t5465 + 0.1478346675e-2_f64 * t344 * t5483 - 0.65704296666666666667e-3_f64 * t5486 - 0.65704296666666666667e-3_f64 * t1102 * t5490 - 0.1478346675e-2_f64 * t1102 * t5495 + 0.19711289e-2_f64 * t1102 * t5500 - 0.98556445e-3_f64 * t344 * t5528 - 4.0_f64 * t1360 * t1924 - 4.0_f64 * t486 * t5623;
    t5626
}
