//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2011/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2011<F: Float>(t10022: F, t14230: F, t2782: F, t10066: F, t10070: F, t10074: F, t10080: F, t10085: F, t10098: F, t10102: F, t14066: F, t14203: F, t14209: F, t14218: F, t14221: F, t14227: F, t14229: F, t213: F, t546: F) -> (F, F, F) {
    let t14231 = t10022 * t14230;
    let t14233 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t14231;
    let t14237 = -F::cast_from(0.65049603595885220126e-3_f64) * t14203 + t14209 + F::cast_from(0.54878743191129263322e-2_f64) * t10066 - F::cast_from(0.14634331517634470219e-1_f64) * t10070 + F::cast_from(0.13009920719177044025e-2_f64) * t10074 + F::cast_from(0.10975748638225852664e-1_f64) * t10080 + F::cast_from(0.54878743191129263322e-2_f64) * t10085 - t14218 - F::cast_from(0.11565819519348392139e-2_f64) * t14221 - F::cast_from(0.2601984143835408805e-1_f64) * t10098 + t10102 + t14227 - t14229 - t14233 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t14066;
    (t14231, t14233, t14237)
}
