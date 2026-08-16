//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3306/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3306(t10530: f64, t18718: f64, t2470: f64, t18719: f64, t39609: f64, t18761: f64, t874: f64, t14602: f64, t2482: f64, t2811: f64, t5977: f64, t213: f64, t234: f64, t39624: f64, t39633: f64, t39635: f64, t39640: f64, t51339: f64, t51355: f64, t51371: f64, t51373: f64, t62509: f64) -> f64 {
    let t62665 = t10530 * t18718 * t2470;
    let t62667 = t39609 * t18719;
    let t62670 = t874 * t18761 * t2470;
    let t62675 = t2482 * t2811 * t5977 * t14602;
    let t62679 = -0.22089088168956307394e-3_f64 * t39624 + 0.19514881078765566038e-1_f64 * t51339 - 0.2601984143835408805e-2_f64 * t51355 + t39633 + 0.60712963356159538784e-1_f64 * t39635 + 0.65854491829355115987e0_f64 * t213 * t234 * t62509 - 0.26019841438354088049e-1_f64 * t62665 + 0.39029762157531132074e-1_f64 * t62667 - 0.13009920719177044025e-1_f64 * t62670 - 0.19514881078765566038e-1_f64 * t51371 + 0.11708928647259339622e0_f64 * t62675 - 0.29268663035268940438e-1_f64 * t51373 - 0.11565819519348392139e-2_f64 * t39640;
    t62679
}
