//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1222/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1222<F: Float>(t43083: F, t43086: F, t43088: F, t43090: F, t43092: F, t43094: F, t43097: F, t43100: F, t43103: F, t43105: F, t43108: F, t43111: F) -> F {
    let t44229 = -F::cast_from(0.20803732176130244552e1_f64) * t43083 + F::cast_from(0.86682217400542685632e-1_f64) * t43086 - F::cast_from(0.54878743191129263322e-1_f64) * t43088 - F::cast_from(0.10975748638225852664e0_f64) * t43090 + F::cast_from(0.10975748638225852664e0_f64) * t43092 + F::cast_from(0.17336443480108537126e0_f64) * t43094 - F::cast_from(0.95219938395347901947e-2_f64) * t43097 + F::cast_from(0.47609969197673950973e-2_f64) * t43100 - F::cast_from(0.28565981518604370584e-1_f64) * t43103 + F::cast_from(0.95219938395347901947e-2_f64) * t43105 - F::cast_from(0.10401866088065122276e1_f64) * t43108 + F::cast_from(0.47609969197673950973e-2_f64) * t43111;
    t44229
}
