//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 474/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk474(t1421: f64, t338: f64, t3519: f64, t3522: f64, t3524: f64, t3526: f64, t3535: f64, t3541: f64, t3546: f64, t3551: f64, t3555: f64, t3561: f64, t3567: f64, t3589: f64, t3595: f64, t3621: f64, t3624: f64, t3729: f64, t456: f64) -> f64 {
    let t3732 = -t3519 + 0.8760572888888888889e-3_f64 * t3522 + 0.19711289e-2_f64 * t3524 - 0.13140859333333333333e-2_f64 * t3526 + 0.10950716111111111111e-2_f64 * t1421 * t3535 + 0.19711289e-2_f64 * t1421 * t3541 - 0.13140859333333333333e-2_f64 * t1421 * t3546 - 0.13140859333333333333e-2_f64 * t1421 * t3551 + 0.65704296666666666667e-3_f64 * t1421 * t3555 + 0.7391733375e-3_f64 * t456 * t3561 - 0.295669335e-2_f64 * t1421 * t3567 + 0.1478346675e-2_f64 * t456 * t3589 + 0.19711289e-2_f64 * t456 * t3595 - 0.98556445e-3_f64 * t456 * t3621 - 4.0_f64 * t3624 - 4.0_f64 * t338 * t3729;
    t3732
}
