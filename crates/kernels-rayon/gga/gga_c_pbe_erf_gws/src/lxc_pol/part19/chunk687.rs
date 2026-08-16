//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 687/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk687(t3861: f64, t824: f64, t905: f64, t3717: f64, t858: f64, t886: f64, t884: f64, t904: f64, t933: f64, t2300: f64, t3703: f64, t3855: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3862 = t3861 * t824;
    let t3863 = t905 * t3862;
    let t3866 = t858 * t3717;
    let t3867 = t886 * t3866;
    let t3869 = t884 * t3867 / 48.0_f64;
    let t3871 = t933 * t904 * t3717;
    let t3875 = t2300 * t904 * t3703;
    let t3879 = t858 * t3855;
    (t3862, t3863, t3867, t3869, t3871, t3875, t3879)
}
