//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2818/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2818<F: Float>(t18719: F, t51549: F, t23245: F, t2798: F, t686: F, t72: F, t23359: F, t874: F, t10871: F, t6016: F, t14495: F, t14502: F, t14546: F, t14587: F, t18699: F, t23160: F, t23168: F, t39649: F, t40258: F, t4494: F, t4504: F, t4514: F, t51374: F, t62682: F, t76131: F, t820: F, t836: F, t837: F) -> (F, F, F) {
    let t76206 = t51549 * t18719;
    let t76223 = t2798 * t23245 * t72 * t686;
    let t76237 = t874 * t23359 * t72 * t686;
    let t76242 = t10871 * t6016;
    let t76247 = F::cast_from(0.11853808529283920877e2_f64) * t4504 * t18699 * t14587 - t51374 - F::cast_from(0.9757440539382783019e-2_f64) * t76223 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t76131 * t837 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t14502 * t23160 - F::cast_from(0.29272321618148349057e-1_f64) * t62682 - F::cast_from(0.39512695097613069591e1_f64) * t820 * t40258 * t23168 + F::cast_from(0.9757440539382783019e-2_f64) * t76237 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t18699 * t14495 - F::cast_from(0.11853808529283920877e2_f64) * t14546 * t4494 * t76242 * t836 + t39649;
    (t76206, t76242, t76247)
}
