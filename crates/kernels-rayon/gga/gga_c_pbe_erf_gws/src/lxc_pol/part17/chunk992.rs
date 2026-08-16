//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 992/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk992(t1133: f64, t2157: f64, t810: f64, t874: f64, t4386: f64, t3138: f64, t1105: f64, t2171: f64, t8599: f64, t2168: f64, t1134: f64, t858: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8884 = t1133 * t2157;
    let t8885 = t874 * t810;
    let t8886 = t8884 * t8885;
    let t8887 = t4386 * t8886;
    let t8889 = t3138 * t8887 / 12.0_f64;
    let t8890 = t1105 * t874;
    let t8891 = t8890 * t2171;
    let t8892 = t8599 * t8891;
    let t8894 = t2168 * t8892 / 8.0_f64;
    let t8895 = t1134 * t810;
    let t8896 = t858 * t8895;
    (t8884, t8886, t8889, t8890, t8891, t8894, t8896)
}
