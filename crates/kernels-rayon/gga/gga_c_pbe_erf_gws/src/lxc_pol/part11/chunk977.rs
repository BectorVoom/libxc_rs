//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 977/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk977(t1778: f64, t3493: f64, t17470: f64, t1820: f64, t3534: f64, t2615: f64, t7579: f64, t3526: f64, t4991: f64, t587: f64, t1986: f64, t3459: f64) -> (f64, f64, f64, f64, f64) {
    let t32670 = t3493 * t1778;
    let t32704 = t1820 * t17470 * t3534;
    let t32710 = t2615 * t7579;
    let t32739 = t587 * t4991 * t3526;
    let t32759 = t3459 * t1986;
    (t32670, t32704, t32710, t32739, t32759)
}
