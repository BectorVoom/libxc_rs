//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1139/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1139<F: Float>(t10995: F, t14991: F, t11044: F, t4481: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14489: F, t14979: F, t14985: F, t14989: F, t865: F, t2435: F, t4477: F) -> (F, F) {
    let t14992 = t10995 * t14991;
    let t14995 = 0.19514881078765566038e-1 * t11044 * t4481;
    let t14997 = -0.65049603595885220126e-3 * t14474 - t14479 - t14484 + 0.13009920719177044025e-1 * t14486 - 0.39512695097613069591e1 * t865 * t14489 - 0.65854491829355115987e0 * t865 * t14979 - t14985 - t14989 + 0.39029762157531132075e-1 * t14992 - t14995 + 0.14634331517634470219e-1 * t10498 + t10501;
    let t14998 = t2435 * t4477;
    (t14997, t14998)
}
