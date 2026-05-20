//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2733/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2733<F: Float>(t10509: F, t10995: F, t14990: F, t122: F, t14982: F, t2466: F, t11008: F, t11009: F, t1579: F, t2771: F, t40999: F, t41003: F, t41004: F, t41006: F, t41014: F, t41018: F, t41021: F, t41026: F, t41029: F, t41032: F, t41034: F, t41037: F, t41078: F, t4533: F, t865: F) -> F {
    let t50253 = t10995 * t14990 * t10509;
    let t50259 = t10995 * t14982 * t122 * t2466;
    let t50276 = -F::cast_from(0.43902994552903410657e-1_f64) * t40999 - F::cast_from(0.7805952431506226415e-1_f64) * t50253 - t41003 + F::cast_from(0.51220160311720645767e-1_f64) * t41004 + F::cast_from(0.43902994552903410657e-1_f64) * t41006 + F::cast_from(0.11708928647259339623e0_f64) * t50259 + F::cast_from(0.69394917116090352834e-2_f64) * t41014 - F::cast_from(0.29272321618148349057e-1_f64) * t41018 - F::cast_from(0.69394917116090352834e-2_f64) * t41021 + F::cast_from(0.9757440539382783019e-2_f64) * t41026 + F::cast_from(0.34697458558045176417e-2_f64) * t41029 - F::cast_from(0.32927245914677557992e-1_f64) * t41032 + F::cast_from(0.7805952431506226415e-2_f64) * t41034 + t41037 - F::cast_from(0.11853808529283920877e2_f64) * t865 * t11008 * t4533 * t2771 + F::cast_from(0.15805078039045227836e2_f64) * t865 * t41078 * t1579 * t11009;
    t50276
}
