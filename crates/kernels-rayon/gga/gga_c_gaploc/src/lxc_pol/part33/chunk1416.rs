//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1416/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1416(t28129: f64, t28150: f64, t32907: f64, t32910: f64, t32911: f64, t32923: f64, t32925: f64, t32927: f64, t32928: f64, t32931: f64, t32935: f64, t32936: f64, t32938: f64, t32940: f64, t32942: f64) -> f64 {
    let t38958 = -t32907 + t32910 - t32911 + t32923 + t32925 + t32927 + t28129 + t32928 + t32931 - 0.76685851907841499354e0_f64 * t28150 + t32935 + t32936 + t32938 + t32940 + t32942;
    t38958
}
