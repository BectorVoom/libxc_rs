//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1232/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1232(t32482: f64, t10749: f64, t731: f64, t23362: f64, t2936: f64, t5269: f64, t10755: f64, t5293: f64, t27403: f64, t954: f64, t32179: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32483 = 0.22430701504581487494e-2_f64 * t32482;
    let t32484 = t731 * t10749;
    let t32485 = 0.85450291446024714264e-3_f64 * t32484;
    let t32488 = 0.46143157380853345702e-1_f64 * t5269 * t2936 * t23362;
    let t32490 = 0.20508069947045931424e-1_f64 * t5293 * t10755;
    let t32493 = 0.15381052460284448567e-1_f64 * t5269 * t954 * t27403;
    let t32504 = t550 * t32179;
    (t32483, t32485, t32488, t32490, t32493, t32504)
}
