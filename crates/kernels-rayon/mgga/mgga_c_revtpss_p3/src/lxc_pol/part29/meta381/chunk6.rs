//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1368/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1368(t10022: f64, t14230: f64, t2782: f64, t10066: f64, t10070: f64, t10074: f64, t10080: f64, t10085: f64, t10098: f64, t10102: f64, t14066: f64, t14203: f64, t14209: f64, t14218: f64, t14221: f64, t14227: f64, t14229: f64, t213: f64, t546: f64) -> f64 {
    let t14231 = t10022 * t14230;
    let t14233 = 0.21951497276451705328e-1_f64 * t2782 * t14231;
    let t14237 = -0.65049603595885220126e-3_f64 * t14203 + t14209 + 0.54878743191129263322e-2_f64 * t10066 - 0.14634331517634470219e-1_f64 * t10070 + 0.13009920719177044025e-2_f64 * t10074 + 0.10975748638225852664e-1_f64 * t10080 + 0.54878743191129263322e-2_f64 * t10085 - t14218 - 0.11565819519348392139e-2_f64 * t14221 - 0.2601984143835408805e-1_f64 * t10098 + t10102 + t14227 - t14229 - t14233 + 0.65854491829355115987e0_f64 * t213 * t546 * t14066;
    t14237
}
