//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2733/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2733(t10509: f64, t10995: f64, t14990: f64, t122: f64, t14982: f64, t2466: f64, t11008: f64, t11009: f64, t1579: f64, t2771: f64, t40999: f64, t41003: f64, t41004: f64, t41006: f64, t41014: f64, t41018: f64, t41021: f64, t41026: f64, t41029: f64, t41032: f64, t41034: f64, t41037: f64, t41078: f64, t4533: f64, t865: f64) -> f64 {
    let t50253 = t10995 * t14990 * t10509;
    let t50259 = t10995 * t14982 * t122 * t2466;
    let t50276 = -0.43902994552903410657e-1_f64 * t40999 - 0.7805952431506226415e-1_f64 * t50253 - t41003 + 0.51220160311720645767e-1_f64 * t41004 + 0.43902994552903410657e-1_f64 * t41006 + 0.11708928647259339623e0_f64 * t50259 + 0.69394917116090352834e-2_f64 * t41014 - 0.29272321618148349057e-1_f64 * t41018 - 0.69394917116090352834e-2_f64 * t41021 + 0.9757440539382783019e-2_f64 * t41026 + 0.34697458558045176417e-2_f64 * t41029 - 0.32927245914677557992e-1_f64 * t41032 + 0.7805952431506226415e-2_f64 * t41034 + t41037 - 0.11853808529283920877e2_f64 * t865 * t11008 * t4533 * t2771 + 0.15805078039045227836e2_f64 * t865 * t41078 * t1579 * t11009;
    t50276
}
