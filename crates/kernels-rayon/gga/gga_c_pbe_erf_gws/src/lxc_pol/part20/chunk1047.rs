//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1047/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1047(t1109: f64, t343: f64, t874: f64, t2118: f64, t9499: f64, t824: f64, t8994: f64, t3038: f64, t3747: f64, t905: f64, t1113: f64, t9856: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11744 = t1109 * t874 * t343;
    let t11745 = t2118 * t11744;
    let t11746 = t9499 * t11745;
    let t11749 = t824 * t8994;
    let t11750 = t9499 * t11749;
    let t11753 = t3038 * t3747;
    let t11754 = t905 * t11753;
    let t11757 = t1113 * t9856;
    (t11744, t11745, t11746, t11749, t11750, t11753, t11754, t11757)
}
