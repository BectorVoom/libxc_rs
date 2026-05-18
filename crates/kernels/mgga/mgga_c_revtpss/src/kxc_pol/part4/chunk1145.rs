//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1145/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1145<F: Float>(t14143: F, t14144: F, t14141: F, t10069: F, t5737: F, t10015: F, t10020: F, t10027: F, t10032: F, t10035: F, t10041: F, t10044: F, t14116: F, t14120: F, t14126: F, t14131: F, t4004: F, t5735: F, t5745: F, t9840: F) -> F {
    let t14145 = t14143 * t14144;
    let t14146 = t14141 * t14145;
    let t14149 = t10069 * t5737;
    let t14151 = -t14116 - F::new(0.19514881078765566038e-1) * t10015 - F::new(0.9757440539382783019e-2) * t10020 + F::new(0.19514881078765566038e-1) * t10027 + F::new(0.65049603595885220126e-3) * t14120 + t14126 + t14131 + F::new(0.13170898365871023197e1) * t5745 * t5735 * t9840 + F::new(0.39512695097613069591e1) * t5745 * t5735 * t4004 + F::new(0.14634331517634470219e-1) * t10032 + t10035 - F::new(0.54878743191129263322e-2) * t10041 + F::new(0.39029762157531132075e-1) * t14146 - F::new(0.13009920719177044025e-2) * t10044 - F::new(0.73171657588172351096e-2) * t14149;
    t14151
}
