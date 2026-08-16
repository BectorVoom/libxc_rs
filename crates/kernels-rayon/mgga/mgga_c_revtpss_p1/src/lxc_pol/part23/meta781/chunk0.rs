//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2588/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2588(t11262: f64, t3711: f64, t5278: f64, t12640: f64, t1811: f64, t3766: f64, t5216: f64, t13141: f64, t1770: f64, t13126: f64, t12050: f64, t17710: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59426 = t3711 * t11262 * t5278;
    let t59464 = t12640 * t1811;
    let t59492 = t5216 * t3766;
    let t59498 = t1770 * t13141;
    let t59550 = t1770 * t13126;
    let t59650 = t17710 * t12050;
    (t59426, t59464, t59492, t59498, t59550, t59650)
}
