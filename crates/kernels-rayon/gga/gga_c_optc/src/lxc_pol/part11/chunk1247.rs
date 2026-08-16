//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1247/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1247(t106: f64, t1299: f64, t13300: f64, t13307: f64, t13316: f64, t16456: f64, t16460: f64, t16568: f64, t167: f64, t2106: f64, t22933: f64, t28700: f64, t3454: f64, t3461: f64, t3462: f64, t38463: f64, t4675: f64, t4723: f64, t48629: f64, t56122: f64, t56187: f64, t56227: f64, t56350: f64, t56373: f64, t56380: f64, t56422: f64, t56457: f64, t56489: f64, t56521: f64, t56553: f64, t56587: f64, t56610: f64, t56631: f64, t670: f64) -> f64 {
    let t56638 = 0.27818116767324025134e1_f64 * t106 * (t56122 + t56187 + t56227 + t56350) * t167 - 0.11127246706929610054e2_f64 * t106 * t48629 * t1299 + 0.33381740120788830161e2_f64 * t106 * t38463 * t4675 - 0.1669087006039441508e2_f64 * t106 * t13300 * t4723 - 0.66763480241577660323e2_f64 * t106 * t28700 * t16456 + 0.66763480241577660323e2_f64 * t13307 * t16460 - 0.11127246706929610054e2_f64 * t106 * t3454 * t16568 + 0.6676348024157766032e2_f64 * t106 * t22933 * t56373 - 0.10014522036236649048e3_f64 * t3461 * t13316 * t4723 + 0.16690870060394415081e2_f64 * t106 * t2106 * t56380 + 0.22254493413859220108e2_f64 * t3461 * t3462 * t16568 - 0.27818116767324025134e1_f64 * t106 * t670 * (t56422 + t56457 + t56489 + t56521 + t56553 + t56587 + t56610 + t56631);
    t56638
}
