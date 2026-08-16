//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1108/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1108(t41041: f64, t41057: f64, t40993: f64, t41001: f64, t41004: f64, t41021: f64, t41024: f64, t41029: f64, t41033: f64, t41037: f64, t41045: f64, t41049: f64, t41053: f64) -> f64 {
    let t44110 = 0.36366215538993788974e-1_f64 * t41041;
    let t44114 = 0.10909864661698136692e0_f64 * t41057;
    let t44115 = 0.35922725105591425692e0_f64 * t40993 - 0.8182398496273602519e0_f64 * t41001 - 0.16364796992547205038e0_f64 * t41004 + 0.66671395154821946452e-1_f64 * t41021 - 0.40911992481368012596e-1_f64 * t41024 - 0.20001418546446583936e0_f64 * t41029 + 0.26668558061928778581e0_f64 * t41033 - 0.14546486215597515589e0_f64 * t41037 - t44110 - 0.20455996240684006298e-1_f64 * t41045 + 0.2727466165424534173e-1_f64 * t41049 + 0.68186654135613354325e-2_f64 * t41053 + t44114;
    t44115
}
