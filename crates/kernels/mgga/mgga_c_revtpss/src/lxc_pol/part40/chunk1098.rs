//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1098/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1098<F: Float>(t13926: F, t543: F, t4100: F, t2782: F, t10014: F, t5741: F, t13790: F, t1398: F, t10022: F, t10066: F, t10070: F, t10074: F, t10080: F, t10085: F, t10098: F, t10102: F, t14066: F, t14203: F, t14209: F, t14218: F, t14221: F, t213: F, t546: F) -> (F,) {
    let t14224 = t13926 * t543;
    let t14225 = t4100 * t14224;
    let t14227 = 0.10975748638225852664e-1 * t2782 * t14225;
    let t14229 = 0.19514881078765566038e-1 * t10014 * t5741;
    let t14230 = t13790 * t1398;
    let t14231 = t10022 * t14230;
    let t14233 = 0.21951497276451705328e-1 * t2782 * t14231;
    let t14237 = -0.65049603595885220126e-3 * t14203 + t14209 + 0.54878743191129263322e-2 * t10066 - 0.14634331517634470219e-1 * t10070 + 0.13009920719177044025e-2 * t10074 + 0.10975748638225852664e-1 * t10080 + 0.54878743191129263322e-2 * t10085 - t14218 - 0.11565819519348392139e-2 * t14221 - 0.2601984143835408805e-1 * t10098 + t10102 + t14227 - t14229 - t14233 + 0.65854491829355115987e0 * t213 * t546 * t14066;
    (t14237,)
}
