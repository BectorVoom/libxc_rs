//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 962/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk962(t2610: f64, t38907: f64, t2033: f64, t2365: f64, t13870: f64, t296: f64, t1: f64, t787: f64, t2028: f64, t325: f64, t550: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47178 = t2610 * t38907;
    let t47180 = t2033 * t2365 * t47178;
    let t47182 = t296 * t13870;
    let t47184 = t787 * t47182 * t1;
    let t47186 = 0.39722766613167140743e-1_f64 * t47184 * t2028;
    let t47187 = t325 * t13870;
    let t47188 = t550 * t47187;
    let t47191 = 0.39722766613167140743e-1_f64 * t2033 * t549 * t47188;
    (t47178, t47180, t47182, t47186, t47187, t47188, t47191)
}
