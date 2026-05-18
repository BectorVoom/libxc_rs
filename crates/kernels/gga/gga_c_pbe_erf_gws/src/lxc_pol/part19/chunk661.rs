//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 661/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk661<F: Float>(t101: F, t3685: F, t171: F, t3379: F, t1342: F, t1349: F, t1386: F, t1388: F, t145: F, t1503: F, t169: F, t242: F, t279: F, t281: F, t2926: F, t296: F, t2986: F, t2990: F, t2996: F, t3003: F, t3007: F, t3015: F, t3373: F, t3617: F, t3620: F, t3626: F, t3638: F, t3642: F, t3645: F, t475: F, t526: F, t988: F) -> (F, F, F) {
    let t3686 = t101 * t3685;
    let t3689 = t171 * t3379;
    let t3700 = t3617 * t279 - t988 * t3620 + F::new(6.0) * t2986 * t2990 - F::new(0.10809180959278284142e0) * t2926 - F::new(0.11974234010254609094e-1) * t281 * t3626 + F::new(3.0) * t475 * t3638 + t988 * t3642 + F::new(6.0) * t1503 * t3645 + t3686 * t526 + (-t1342 + F::new(0.1061188859155979109e0) * t2996 + t1349 - F::new(0.31835665774679373271e-1) * t169 * t3689 * t242 - F::new(0.63671331549358746542e-1) * t3003 - t1386 + t1388 - F::new(0.2133002709687175212e0) * t3007 + F::new(0.533250677421793803e-1) * t145 * t3373) * t296 - F::new(0.58113483035773838734e-3) * t3015;
    (t3686, t3689, t3700)
}
