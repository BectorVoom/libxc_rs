//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1140/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1140(t13984: f64, t14657: f64, t3111: f64, t3950: f64, t850: f64, t833: f64, t1123: f64, t13815: f64, t2397: f64, t4127: f64, t2249: f64, t904: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14658 = t14657 * t13984;
    let t14673 = t850 * t3111 * t3950;
    let t14674 = t14673 * t833;
    let t14677 = t850 * t1123 * t13815;
    let t14678 = t14677 * t833;
    let t14680 = t4127 * t2397;
    let t14682 = t904 * t2249;
    (t14658, t14673, t14674, t14677, t14678, t14680, t14682)
}
