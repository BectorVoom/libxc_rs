//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1294/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1294<F: Float>(t16590: F, t3766: F, t1098: F, t5528: F, t1102: F, t11646: F, t11690: F, t11708: F, t11710: F, t11721: F, t11723: F, t11725: F, t11767: F, t1360: F, t16562: F, t16563: F, t16567: F, t16570: F, t16574: F, t16579: F, t16584: F, t16587: F, t1924: F, t3951: F, t5623: F) -> F {
    let t16591 = t3766 * t16590;
    let t16601 = F::cast_from(0.13140859333333333333e-2_f64) * t1098 * t5528;
    let t16607 = -F::cast_from(0.19711289e-2_f64) * t11646 + t16562 + F::cast_from(0.10950716111111111111e-2_f64) * t1102 * t16563 + t16567 + F::cast_from(0.19711289e-2_f64) * t1102 * t16570 + F::cast_from(0.98556445e-3_f64) * t1102 * t16574 + F::cast_from(0.16426074166666666667e-2_f64) * t1102 * t16579 - F::cast_from(0.1478346675e-2_f64) * t1102 * t16584 - F::cast_from(0.8760572888888888889e-3_f64) * t16587 - F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t16591 - F::cast_from(0.8760572888888888889e-3_f64) * t11690 + F::cast_from(0.492782225e-3_f64) * t11708 + F::cast_from(0.13140859333333333333e-2_f64) * t11710 - F::cast_from(0.65704296666666666666e-3_f64) * t11721 + F::cast_from(0.43802864444444444444e-3_f64) * t11723 + F::cast_from(0.98556445e-3_f64) * t11725 - t16601 - F::cast_from(0.65704296666666666667e-3_f64) * t11767 - F::cast_from(8.0_f64) * t1360 * t5623 - F::cast_from(4.0_f64) * t3951 * t1924;
    t16607
}
