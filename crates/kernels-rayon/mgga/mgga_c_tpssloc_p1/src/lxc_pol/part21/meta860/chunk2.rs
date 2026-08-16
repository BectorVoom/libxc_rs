//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3121/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3121(t14842: f64, t4869: f64, t11292: f64, t6084: f64, t1164: f64, t3404: f64, t3637: f64, t43706: f64, t4700: f64, t6274: f64, t63566: f64, t63568: f64, t63571: f64, t63574: f64, t63576: f64, t63579: f64, t63582: f64, t63585: f64, t63587: f64, t63591: f64, t63594: f64) -> (f64, f64, f64) {
    let t64536 = 0.2077903092681775651e3_f64 * t4869 * t14842;
    let t64537 = t11292 * t6084;
    let t64540 = 0.10389515463408878255e3_f64 * t1164 * t64537 * t3404;
    let t64545 = -6.0_f64 * t3637 * t43706 * t4700 * t6274 - t63566 - t63568 - t63571 - t63574 - t63576 - t63579 - t63582 - t63585 + t63587 + t63591 + t63594 + t64536 + t64540;
    (t64536, t64540, t64545)
}
