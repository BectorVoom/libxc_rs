//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 858/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk858(t353: f64, t8688: f64, t859: f64, t1162: f64, t810: f64, t4386: f64, t1118: f64, t814: f64, t2501: f64, t2370: f64, t830: f64, t1105: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8689 = t353 * t8688;
    let t8690 = t859 * t8689;
    let t8693 = t1162 * t810;
    let t8694 = t353 * t8693;
    let t8695 = t4386 * t8694;
    let t8698 = t1118 * t814;
    let t8699 = t353 * t8698;
    let t8700 = t4386 * t8699;
    let t8708 = t2501 * t810;
    let t8710 = t2370 * t830 * t8708;
    let t8713 = t898 * t1105;
    (t8690, t8695, t8700, t8708, t8710, t8713)
}
