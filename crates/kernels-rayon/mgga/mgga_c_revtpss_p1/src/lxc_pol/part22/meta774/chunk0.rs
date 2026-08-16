//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2861/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2861(t3718: f64, t3722: f64, t44546: f64, t3566: f64, t3766: f64, t5330: f64, t12831: f64, t12865: f64, t1209: f64, t13141: f64, t17708: f64, t11249: f64, t3601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44548 = t3718 * t44546 * t3722;
    let t44550 = t3566 * t3766;
    let t44551 = t44550 * t5330;
    let t44561 = t12831 * t12865;
    let t44578 = t1209 * t13141 * t17708;
    let t44585 = t3601 * t11249;
    (t44548, t44550, t44551, t44561, t44578, t44585)
}
