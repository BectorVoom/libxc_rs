//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2550/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2550(t13965: f64, t3109: f64, t1041: f64, t13969: f64, t14173: f64, t247: f64, t677: f64) -> (f64, f64, f64) {
    let t49831 = t3109 * t13965;
    let t49846 = t1041 * t13969 * t14173;
    let t49850 = t247 * t677;
    (t49831, t49846, t49850)
}
