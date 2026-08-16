//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3915/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3915(t4086: f64, t6888: f64, t786: f64, t4104: f64, t213: f64, t22005: f64, t4004: f64, t47403: f64, t47413: f64, t47417: f64, t47432: f64, t49354: f64, t49361: f64, t49378: f64, t49382: f64, t49386: f64, t49395: f64, t49399: f64, t546: f64, t5745: f64, t74724: f64) -> f64 {
    let t75251 = t786 * t4086 * t6888;
    let t75252 = t75251 * t4104;
    let t75263 = 0.22089088168956307394e-3_f64 * t49354 + 0.14634331517634470219e-1_f64 * t47403 - 0.13009920719177044025e-2_f64 * t47413 + 0.39274398764404314548e-3_f64 * t49361 + 0.65854491829355115987e0_f64 * t213 * t546 * t74724 - t47417 - 0.19514881078765566038e-1_f64 * t75252 + 0.21951497276451705328e-1_f64 * t49378 + 0.10975748638225852664e-1_f64 * t49382 - 0.19514881078765566038e-1_f64 * t49386 + 0.23131639038696784278e-2_f64 * t47432 + 0.65854491829355115984e-1_f64 * t49395 + 0.92196288561097162379e1_f64 * t5745 * t22005 * t4004 + 0.11708928647259339623e0_f64 * t49399;
    t75263
}
