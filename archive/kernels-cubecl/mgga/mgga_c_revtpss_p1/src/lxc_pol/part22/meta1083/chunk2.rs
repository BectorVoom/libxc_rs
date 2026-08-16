//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3915/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3915<F: Float>(t4086: F, t6888: F, t786: F, t4104: F, t213: F, t22005: F, t4004: F, t47403: F, t47413: F, t47417: F, t47432: F, t49354: F, t49361: F, t49378: F, t49382: F, t49386: F, t49395: F, t49399: F, t546: F, t5745: F, t74724: F) -> F {
    let t75251 = t786 * t4086 * t6888;
    let t75252 = t75251 * t4104;
    let t75263 = F::cast_from(0.22089088168956307394e-3_f64) * t49354 + F::cast_from(0.14634331517634470219e-1_f64) * t47403 - F::cast_from(0.13009920719177044025e-2_f64) * t47413 + F::cast_from(0.39274398764404314548e-3_f64) * t49361 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t74724 - t47417 - F::cast_from(0.19514881078765566038e-1_f64) * t75252 + F::cast_from(0.21951497276451705328e-1_f64) * t49378 + F::cast_from(0.10975748638225852664e-1_f64) * t49382 - F::cast_from(0.19514881078765566038e-1_f64) * t49386 + F::cast_from(0.23131639038696784278e-2_f64) * t47432 + F::cast_from(0.65854491829355115984e-1_f64) * t49395 + F::cast_from(0.92196288561097162379e1_f64) * t5745 * t22005 * t4004 + F::cast_from(0.11708928647259339623e0_f64) * t49399;
    t75263
}
