//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2818/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2818(t18719: f64, t51549: f64, t23245: f64, t2798: f64, t686: f64, t72: f64, t23359: f64, t874: f64, t10871: f64, t6016: f64, t14495: f64, t14502: f64, t14546: f64, t14587: f64, t18699: f64, t23160: f64, t23168: f64, t39649: f64, t40258: f64, t4494: f64, t4504: f64, t4514: f64, t51374: f64, t62682: f64, t76131: f64, t820: f64, t836: f64, t837: f64) -> (f64, f64, f64) {
    let t76206 = t51549 * t18719;
    let t76223 = t2798 * t23245 * t72 * t686;
    let t76237 = t874 * t23359 * t72 * t686;
    let t76242 = t10871 * t6016;
    let t76247 = 0.11853808529283920877e2_f64 * t4504 * t18699 * t14587 - t51374 - 0.9757440539382783019e-2_f64 * t76223 - 0.19756347548806534796e1_f64 * t4514 * t76131 * t837 + 0.39512695097613069591e1_f64 * t4504 * t14502 * t23160 - 0.29272321618148349057e-1_f64 * t62682 - 0.39512695097613069591e1_f64 * t820 * t40258 * t23168 + 0.9757440539382783019e-2_f64 * t76237 - 0.19756347548806534796e1_f64 * t4514 * t18699 * t14495 - 0.11853808529283920877e2_f64 * t14546 * t4494 * t76242 * t836 + t39649;
    (t76206, t76242, t76247)
}
