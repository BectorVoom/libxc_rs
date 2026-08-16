//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 889/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk889(t2585: f64, t5312: f64, t1648: f64, t2689: f64, t2566: f64, t5129: f64, t587: f64, t2768: f64, t562: f64, t7694: f64, t1820: f64, t2620: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t7710 = 16.0_f64 / 45.0_f64 * t5312 * t2585;
    let t7712 = 8.0_f64 / 45.0_f64 * t1648 * t2689;
    let t7713 = t5129 * t2566;
    let t7715 = 16.0_f64 / 135.0_f64 * t587 * t7713;
    let t7716 = t2768 * t562;
    let t7717 = t7694 * t7716;
    let t7719 = 32.0_f64 / 45.0_f64 * t1820 * t7717;
    let t7720 = t2620 * t597;
    (t7710, t7712, t7715, t7719, t7720)
}
