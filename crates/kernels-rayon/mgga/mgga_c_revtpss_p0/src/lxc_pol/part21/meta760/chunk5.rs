//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2689/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2689(t1399: f64, t14193: f64, t46422: f64, t47450: f64, t47454: f64, t47455: f64, t48760: f64, t49213: f64, t49429: f64, t49432: f64, t49439: f64, t49446: f64, t49450: f64, t5735: f64, t5745: f64, t5755: f64, t9995: f64) -> f64 {
    let t49456 = 0.19514881078765566037e-2_f64 * t49429 - 0.46263278077393568556e-2_f64 * t49432 + 0.11853808529283920877e2_f64 * t5745 * t5735 * t46422 - 0.13878983423218070567e-1_f64 * t47450 + t47454 + 0.15805078039045227836e2_f64 * t49439 * t5735 * t48760 - 0.23707617058567841754e2_f64 * t14193 * t5735 * t9995 - 0.29272321618148349057e-1_f64 * t49446 + 0.11708928647259339623e0_f64 * t49450 - 0.7805952431506226415e-2_f64 * t47455 - 0.19756347548806534796e1_f64 * t5755 * t49213 * t1399;
    t49456
}
