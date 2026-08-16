//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3099/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3099(t1149: f64, t24324: f64, t3384: f64, t24323: f64, t3435: f64, t3433: f64, t12227: f64, t20651: f64, t5104: f64, t24220: f64, t44091: f64, t44093: f64) -> (f64, f64, f64, f64) {
    let t81649 = 2.0_f64 * t3384 * t24324 * t1149;
    let t81650 = t24323 * t3435;
    let t81653 = 0.16081979498692535067e2_f64 * t3433 * t81650 * t1149;
    let t81656 = 0.1551780387578202009e4_f64 * t12227 * t20651 * t5104;
    let t81660 = 0.24955700379505800916e5_f64 * t44091 * t24220 * t44093 * t1149;
    (t81649, t81653, t81656, t81660)
}
