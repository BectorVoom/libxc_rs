//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 932/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk932(t10442: f64, t418: f64, t2559: f64, t587: f64, t3421: f64, t562: f64, t1820: f64, t1022: f64, t7490: f64, t2679: f64, t5211: f64, t10401: f64, t617: f64, t7499: f64) -> (f64, f64, f64, f64, f64) {
    let t10443 = t10442 * t418;
    let t10444 = t2559 * t10443;
    let t10446 = 4.0_f64 / 27.0_f64 * t587 * t10444;
    let t10447 = t3421 * t562;
    let t10448 = t2559 * t10447;
    let t10450 = 8.0_f64 / 27.0_f64 * t1820 * t10448;
    let t10451 = t7490 * t1022;
    let t10452 = t10451 * t2679;
    let t10454 = 16.0_f64 / 27.0_f64 * t5211 * t10452;
    let t10456 = t7499 * t10401 * t617;
    (t10443, t10446, t10450, t10454, t10456)
}
