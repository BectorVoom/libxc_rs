//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 931/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk931(t13212: f64, t7137: f64, t13203: f64, t32190: f64, t935: f64, t2508: f64, t2580: f64, t13209: f64, t7129: f64, t3431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42998 = 0.30762104920568897135e-1_f64 * t7137 * t13212;
    let t42999 = t7137 * t13203;
    let t43001 = t32190 * t935;
    let t43003 = t2508 * t2580 * t43001;
    let t43006 = 0.76905262301422242837e-2_f64 * t7129 * t13209;
    let t43007 = t3431 * t935;
    (t42998, t42999, t43001, t43003, t43006, t43007)
}
