//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 470/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk470(t1212: f64, t3696: f64, t3697: f64, t3571: f64, t3657: f64, t3573: f64, t3577: f64, t3581: f64, t3585: f64, t3607: f64, t3609: f64, t3652: f64, t3654: f64, t3659: f64, t3663: f64, t3666: f64, t3669: f64) -> (f64, f64) {
    let t3699 = t3696 * t3697 * t1212;
    let t3704 = 0.40256666666666666667e0_f64 * t3571;
    let t3711 = 0.137975e0_f64 * t3657;
    let t3716 = -0.1294625e1_f64 * t3607 + 0.258925e1_f64 * t3609 + t3704 + 0.20128333333333333334e0_f64 * t3573 - 0.20128333333333333333e0_f64 * t3577 + 0.60385e0_f64 * t3581 - 0.301925e0_f64 * t3585 + 0.82524375e-1_f64 * t3652 + 0.16504875e0_f64 * t3654 + t3711 + 0.11038e0_f64 * t3659 - 0.27595e-1_f64 * t3663 + 0.16557e0_f64 * t3666 - 0.82785e-1_f64 * t3669;
    (t3699, t3716)
}
