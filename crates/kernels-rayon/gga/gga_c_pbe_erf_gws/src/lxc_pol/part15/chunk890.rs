//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 890/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk890(t2768: f64, t610: f64, t7720: f64, t587: f64, t2646: f64, t719: f64, t256: f64, t19: f64, t2522: f64, t336: f64, t714: f64, t1061: f64, t1923: f64) -> (f64, f64, f64, f64) {
    let t7721 = t2768 * t610;
    let t7722 = t7720 * t7721;
    let t7724 = 16.0_f64 / 45.0_f64 * t587 * t7722;
    let t7726 = t2646 * t719;
    let t7728 = 2.0_f64 / 3.0_f64 * t7726 * t256;
    let t7729 = t2522 * t19;
    let t7730 = t7729 * t336;
    let t7732 = 0.12155555555555555555e0_f64 * t7730 * t714;
    let t7733 = t1061 * t1923;
    (t7724, t7728, t7732, t7733)
}
