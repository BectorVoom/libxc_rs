//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 975/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk975(t2268: f64, t30456: f64, t1562: f64, t30948: f64, t1444: f64, t1992: f64, t30154: f64, t7586: f64, t1350: f64, t30147: f64, t5129: f64, t7647: f64) -> (f64, f64, f64, f64, f64) {
    let t34510 = t30456 * t2268;
    let t34512 = t30948 * t1562;
    let t34513 = 0.16006300097412701803e-1_f64 * t34512;
    let t34516 = t30154 * t7586 * t1992 * t1444;
    let t34526 = t30147 * t7586 * t1992 * t1350;
    let t34534 = t7647 * t5129;
    (t34510, t34513, t34516, t34526, t34534)
}
