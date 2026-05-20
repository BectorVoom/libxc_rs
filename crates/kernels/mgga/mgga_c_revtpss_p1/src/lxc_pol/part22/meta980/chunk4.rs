//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3306/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3306<F: Float>(t10530: F, t18718: F, t2470: F, t18719: F, t39609: F, t18761: F, t874: F, t14602: F, t2482: F, t2811: F, t5977: F, t213: F, t234: F, t39624: F, t39633: F, t39635: F, t39640: F, t51339: F, t51355: F, t51371: F, t51373: F, t62509: F) -> F {
    let t62665 = t10530 * t18718 * t2470;
    let t62667 = t39609 * t18719;
    let t62670 = t874 * t18761 * t2470;
    let t62675 = t2482 * t2811 * t5977 * t14602;
    let t62679 = -F::cast_from(0.22089088168956307394e-3_f64) * t39624 + F::cast_from(0.19514881078765566038e-1_f64) * t51339 - F::cast_from(0.2601984143835408805e-2_f64) * t51355 + t39633 + F::cast_from(0.60712963356159538784e-1_f64) * t39635 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t234 * t62509 - F::cast_from(0.26019841438354088049e-1_f64) * t62665 + F::cast_from(0.39029762157531132074e-1_f64) * t62667 - F::cast_from(0.13009920719177044025e-1_f64) * t62670 - F::cast_from(0.19514881078765566038e-1_f64) * t51371 + F::cast_from(0.11708928647259339622e0_f64) * t62675 - F::cast_from(0.29268663035268940438e-1_f64) * t51373 - F::cast_from(0.11565819519348392139e-2_f64) * t39640;
    t62679
}
