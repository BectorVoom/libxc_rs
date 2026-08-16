//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 732/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk732(t1324: f64, t225: f64, t2600: f64, t541: f64, t1329: f64, t3726: f64, t119: f64, t3734: f64, t210: f64, t3719: f64, t3752: f64, t554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3758 = t1324 * t225;
    let t3762 = 35.0_f64 / 432.0_f64 * t2600 * t541;
    let t3763 = t3726 * t1329;
    let t3765 = t119 * t3734;
    let t3766 = t210 * t3765;
    let t3770 = t210 * t119 * t3719;
    let t3773 = t3752 * t225;
    let t3774 = t3773 * t554;
    (t3758, t3762, t3763, t3766, t3770, t3773, t3774)
}
