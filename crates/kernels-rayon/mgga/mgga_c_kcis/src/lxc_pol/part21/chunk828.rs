//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 828/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk828(t1088: f64, t3245: f64, t1014: f64, t3171: f64, t1008: f64, t2811: f64, t977: f64, t278: f64, t2835: f64, t975: f64, t119: f64, t251: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10450 = t3245 * t1088;
    let t10452 = t1014 * t3171;
    let t10454 = t1008 * t2811;
    let t10461 = t977 * t977;
    let t10462 = 1.0_f64 / t10461;
    let t10463 = t278 * t10462;
    let t10466 = t975 * t2835;
    let t10470 = t85 * t119 * t251;
    (t10450, t10452, t10454, t10461, t10462, t10463, t10466, t10470)
}
