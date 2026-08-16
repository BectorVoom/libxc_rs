//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 794/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk794(t1086: f64, t2777: f64, t3371: f64, t2811: f64, t3396: f64, t2979: f64, t8117: f64, t3388: f64, t2520: f64, t3392: f64, t128: f64, t147: f64, t19: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9485 = t1086 * t2777;
    let t9486 = t3371 * t9485;
    let t9488 = t3396 * t2811;
    let t9490 = t8117 * t2979;
    let t9491 = t9490 * t3388;
    let t9493 = t2520 * t2979;
    let t9494 = t9493 * t3392;
    let t9497 = t128 * t19 * t147;
    (t9485, t9486, t9488, t9491, t9494, t9497)
}
