//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1250/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1250(t14987: f64, t2467: f64, t122: f64, t4480: f64, t2466: f64, t10995: f64, t11044: f64, t4481: f64, t10498: f64, t10501: f64, t14474: f64, t14479: f64, t14484: f64, t14486: f64, t14489: f64, t14979: f64, t14985: f64, t865: f64) -> f64 {
    let t14989 = 0.19514881078765566038e-1_f64 * t14987 * t2467;
    let t14990 = t4480 * t122;
    let t14991 = t14990 * t2466;
    let t14992 = t10995 * t14991;
    let t14995 = 0.19514881078765566038e-1_f64 * t11044 * t4481;
    let t14997 = -0.65049603595885220126e-3_f64 * t14474 - t14479 - t14484 + 0.13009920719177044025e-1_f64 * t14486 - 0.39512695097613069591e1_f64 * t865 * t14489 - 0.65854491829355115987e0_f64 * t865 * t14979 - t14985 - t14989 + 0.39029762157531132075e-1_f64 * t14992 - t14995 + 0.14634331517634470219e-1_f64 * t10498 + t10501;
    t14997
}
