//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1091/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1091<F: Float>(t555: F, t5658: F, t4086: F, t543: F, t2782: F, t1882: F, t4114: F, t2482: F, t122: F, t4003: F, t72: F, t1398: F, t676: F, t10069: F, t5737: F, t10015: F, t10020: F, t10027: F, t10032: F, t10035: F, t10041: F, t10044: F, t14116: F, t14120: F, t14126: F, t4004: F, t5735: F, t5745: F, t9840: F) -> (F, F) {
    let t14127 = t555 * t5658;
    let t14129 = t4086 * t14127 * t543;
    let t14131 = 0.10975748638225852664e-1 * t2782 * t14129;
    let t14140 = t4114 * t1882;
    let t14141 = t2482 * t14140;
    let t14143 = t4003 * t72 * t122;
    let t14144 = t676 * t1398;
    let t14145 = t14143 * t14144;
    let t14146 = t14141 * t14145;
    let t14149 = t10069 * t5737;
    let t14151 = -t14116 - 0.19514881078765566038e-1 * t10015 - 0.9757440539382783019e-2 * t10020 + 0.19514881078765566038e-1 * t10027 + 0.65049603595885220126e-3 * t14120 + t14126 + t14131 + 0.13170898365871023197e1 * t5745 * t5735 * t9840 + 0.39512695097613069591e1 * t5745 * t5735 * t4004 + 0.14634331517634470219e-1 * t10032 + t10035 - 0.54878743191129263322e-2 * t10041 + 0.39029762157531132075e-1 * t14146 - 0.13009920719177044025e-2 * t10044 - 0.73171657588172351096e-2 * t14149;
    (t14127, t14151)
}
