//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1302/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1302(t18210: f64, t29525: f64, t7968: f64, t1464: f64, t1489: f64, t7392: f64, t98310: f64, t1497: f64, t28503: f64, t60756: f64, t28720: f64, t6140: f64) -> (f64, f64, f64, f64, f64) {
    let t102294 = t18210 * t29525;
    let t102295 = t7968 * t102294;
    let t102299 = t1464 * t98310 * t7392 * t1489;
    let t102303 = t1464 * t28503 * t60756 * t1497;
    let t102305 = t28720 * t6140;
    (t102294, t102295, t102299, t102303, t102305)
}
