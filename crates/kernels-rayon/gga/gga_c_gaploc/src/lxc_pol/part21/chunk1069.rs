//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1069/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1069(t1: f64, t23092: f64, t22044: f64, t739: f64, t21460: f64, t5654: f64, t7802: f64, t10912: f64, t1422: f64, t787: f64, t2672: f64, t6081: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23348 = t23092 * t1;
    let t23362 = t739 * t22044;
    let t23433 = t739 * t21460;
    let t23469 = t5654 * t7802;
    let t23477 = t787 * t10912 * t1422;
    let t23492 = t6081 * t2672;
    (t23348, t23362, t23433, t23469, t23477, t23492)
}
