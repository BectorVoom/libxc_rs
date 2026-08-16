//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 661/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk661(t101: f64, t3685: f64, t171: f64, t3379: f64, t1342: f64, t1349: f64, t1386: f64, t1388: f64, t145: f64, t1503: f64, t169: f64, t242: f64, t279: f64, t281: f64, t2926: f64, t296: f64, t2986: f64, t2990: f64, t2996: f64, t3003: f64, t3007: f64, t3015: f64, t3373: f64, t3617: f64, t3620: f64, t3626: f64, t3638: f64, t3642: f64, t3645: f64, t475: f64, t526: f64, t988: f64) -> (f64, f64, f64) {
    let t3686 = t101 * t3685;
    let t3689 = t171 * t3379;
    let t3700 = t3617 * t279 - t988 * t3620 + 6.0_f64 * t2986 * t2990 - 0.10809180959278284142e0_f64 * t2926 - 0.11974234010254609094e-1_f64 * t281 * t3626 + 3.0_f64 * t475 * t3638 + t988 * t3642 + 6.0_f64 * t1503 * t3645 + t3686 * t526 + (-t1342 + 0.1061188859155979109e0_f64 * t2996 + t1349 - 0.31835665774679373271e-1_f64 * t169 * t3689 * t242 - 0.63671331549358746542e-1_f64 * t3003 - t1386 + t1388 - 0.2133002709687175212e0_f64 * t3007 + 0.533250677421793803e-1_f64 * t145 * t3373) * t296 - 0.58113483035773838734e-3_f64 * t3015;
    (t3686, t3689, t3700)
}
