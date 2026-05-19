//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1207/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1207<F: Float>(t14987: F, t2467: F, t122: F, t4480: F, t2466: F, t10995: F, t11044: F, t4481: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14489: F, t14979: F, t14985: F, t865: F) -> F {
    let t14989 = F::cast_from(0.19514881078765566038e-1_f64) * t14987 * t2467;
    let t14990 = t4480 * t122;
    let t14991 = t14990 * t2466;
    let t14992 = t10995 * t14991;
    let t14995 = F::cast_from(0.19514881078765566038e-1_f64) * t11044 * t4481;
    let t14997 = -F::cast_from(0.65049603595885220126e-3_f64) * t14474 - t14479 - t14484 + F::cast_from(0.13009920719177044025e-1_f64) * t14486 - F::cast_from(0.39512695097613069591e1_f64) * t865 * t14489 - F::cast_from(0.65854491829355115987e0_f64) * t865 * t14979 - t14985 - t14989 + F::cast_from(0.39029762157531132075e-1_f64) * t14992 - t14995 + F::cast_from(0.14634331517634470219e-1_f64) * t10498 + t10501;
    t14997
}
