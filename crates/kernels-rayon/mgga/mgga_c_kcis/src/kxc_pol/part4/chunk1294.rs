//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1294/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1294(t16590: f64, t3766: f64, t1098: f64, t5528: f64, t1102: f64, t11646: f64, t11690: f64, t11708: f64, t11710: f64, t11721: f64, t11723: f64, t11725: f64, t11767: f64, t1360: f64, t16562: f64, t16563: f64, t16567: f64, t16570: f64, t16574: f64, t16579: f64, t16584: f64, t16587: f64, t1924: f64, t3951: f64, t5623: f64) -> f64 {
    let t16591 = t3766 * t16590;
    let t16601 = 0.13140859333333333333e-2_f64 * t1098 * t5528;
    let t16607 = -0.19711289e-2_f64 * t11646 + t16562 + 0.10950716111111111111e-2_f64 * t1102 * t16563 + t16567 + 0.19711289e-2_f64 * t1102 * t16570 + 0.98556445e-3_f64 * t1102 * t16574 + 0.16426074166666666667e-2_f64 * t1102 * t16579 - 0.1478346675e-2_f64 * t1102 * t16584 - 0.8760572888888888889e-3_f64 * t16587 - 0.13140859333333333333e-2_f64 * t1102 * t16591 - 0.8760572888888888889e-3_f64 * t11690 + 0.492782225e-3_f64 * t11708 + 0.13140859333333333333e-2_f64 * t11710 - 0.65704296666666666666e-3_f64 * t11721 + 0.43802864444444444444e-3_f64 * t11723 + 0.98556445e-3_f64 * t11725 - t16601 - 0.65704296666666666667e-3_f64 * t11767 - 8.0_f64 * t1360 * t5623 - 4.0_f64 * t3951 * t1924;
    t16607
}
