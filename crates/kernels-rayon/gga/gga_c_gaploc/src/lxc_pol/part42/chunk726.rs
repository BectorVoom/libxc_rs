//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 726/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk726(t1029: f64, t12255: f64, t1024: f64, t3732: f64, t14364: f64, t169: f64, t299: f64, t706: f64, t13195: f64, t13201: f64, t13537: f64, t13544: f64, t13547: f64, t13550: f64, t13554: f64, t13558: f64, t2508: f64, t270: f64) -> (f64, f64, f64, f64, f64) {
    let t14428 = t12255 * t1029;
    let t14431 = t3732 * t1024;
    let t14435 = t14364 * t169 * t299;
    let t14436 = t706 * t14435;
    let t14439 = t13537 + t13544 - t13547 + t13550 - t13554 + t13558 + 0.25635087433807414279e-2_f64 * t13195 - 0.38452631150711121419e-2_f64 * t13201 - 0.46143157380853345702e-1_f64 * t2508 * t14428 + 0.15381052460284448567e-1_f64 * t2508 * t14431 + 0.76905262301422242837e-2_f64 * t270 * t14436;
    (t14428, t14431, t14435, t14436, t14439)
}
