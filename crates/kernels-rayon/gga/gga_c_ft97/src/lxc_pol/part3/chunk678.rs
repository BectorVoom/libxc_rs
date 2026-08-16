//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 678/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk678(t10279: f64, t10397: f64, t192: f64, t7640: f64, t2842: f64, t863: f64, t869: f64, t309: f64, t2770: f64, t871: f64, t8232: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10640 = 4.0_f64 / 27.0_f64 * t10279;
    let t10658 = 28.0_f64 / 81.0_f64 * t10397;
    let t10683 = t192 * t7640;
    let t10688 = t863 * t2842;
    let t10695 = t869 * t869;
    let t10696 = 1.0_f64 / t10695;
    let t10697 = t309 * t10696;
    let t10703 = t2770 * t871;
    let t10732 = t8232 * t837;
    (t10640, t10658, t10683, t10688, t10697, t10703, t10732)
}
