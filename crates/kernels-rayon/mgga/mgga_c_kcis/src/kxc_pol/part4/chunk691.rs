//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 691/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk691(t1102: f64, t344: f64, t3743: f64, t3744: f64, t3746: f64, t3748: f64, t3757: f64, t3763: f64, t3768: f64, t3773: f64, t3777: f64, t3783: f64, t3789: f64, t3811: f64, t3817: f64, t3843: f64, t3846: f64, t3951: f64, t486: f64) -> f64 {
    let t3954 = -t3743 + 0.8760572888888888889e-3_f64 * t3744 + 0.19711289e-2_f64 * t3746 - 0.13140859333333333333e-2_f64 * t3748 + 0.10950716111111111111e-2_f64 * t1102 * t3757 + 0.19711289e-2_f64 * t1102 * t3763 - 0.13140859333333333333e-2_f64 * t1102 * t3768 - 0.13140859333333333333e-2_f64 * t1102 * t3773 + 0.65704296666666666667e-3_f64 * t1102 * t3777 + 0.7391733375e-3_f64 * t344 * t3783 - 0.295669335e-2_f64 * t1102 * t3789 + 0.1478346675e-2_f64 * t344 * t3811 + 0.19711289e-2_f64 * t344 * t3817 - 0.98556445e-3_f64 * t344 * t3843 - 4.0_f64 * t3846 - 4.0_f64 * t486 * t3951;
    t3954
}
