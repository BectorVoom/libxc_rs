//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1054/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1054(t158: f64, t6393: f64, t1: f64, t544: f64, t2371: f64, t4461: f64, t1397: f64, t6699: f64, t4370: f64, t4389: f64, t1457: f64, t2378: f64) -> (f64, f64, f64, f64, f64) {
    let t21069 = t158 * t6393;
    let t21071 = t544 * t21069 * t1;
    let t21074 = t4461 * t2371;
    let t21077 = t1397 * t6699;
    let t21133 = t544 * t4389 * t4370;
    let t21139 = t1457 * t2378;
    (t21071, t21074, t21077, t21133, t21139)
}
