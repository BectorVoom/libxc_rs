//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 936/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk936(t3562: f64, t626: f64, t422: f64, t1809: f64, t1620: f64, t3553: f64, t1815: f64, t639: f64, t3410: f64, t5125: f64, t1820: f64, t9801: f64) -> (f64, f64, f64, f64) {
    let t10500 = t3562 * t626;
    let t10501 = t10500 * t422;
    let t10502 = t1809 * t10501;
    let t10504 = 8.0_f64 / 45.0_f64 * t1620 * t10502;
    let t10505 = t3553 * t626;
    let t10506 = t10505 * t422;
    let t10507 = t1815 * t10506;
    let t10509 = 4.0_f64 / 45.0_f64 * t639 * t10507;
    let t10510 = t5125 * t3410;
    let t10511 = t1820 * t10510;
    let t10512 = 32.0_f64 / 135.0_f64 * t10511;
    let t10513 = t626 * t9801;
    (t10504, t10509, t10512, t10513)
}
