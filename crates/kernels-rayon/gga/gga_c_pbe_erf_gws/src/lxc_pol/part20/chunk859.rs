//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 859/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk859(t8713: f64, t938: f64, t353: f64, t4386: f64, t2416: f64, t891: f64, t2367: f64, t2503: f64, t1114: f64, t6744: f64, t833: f64, t4423: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8714 = t8713 * t938;
    let t8715 = t353 * t8714;
    let t8716 = t4386 * t8715;
    let t8734 = t891 * t2416;
    let t8740 = 7.0_f64 / 144.0_f64 * t2367 * t2503;
    let t8743 = t1114 * t6744;
    let t8745 = 7.0_f64 / 144.0_f64 * t8743 * t833;
    let t8746 = t1114 * t4423;
    (t8716, t8734, t8740, t8743, t8745, t8746)
}
