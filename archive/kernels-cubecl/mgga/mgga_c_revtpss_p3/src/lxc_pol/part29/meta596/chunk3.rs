//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2006/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2006<F: Float>(t103140: F, t103142: F, t103156: F, t103158: F, t103161: F, t231: F, t4423: F, t7070: F, t7076: F, t7398: F, t7420: F, t95774: F, t95779: F, t95783: F, t95786: F, t95790: F, t95794: F, t95796: F, t95798: F, t99303: F) -> F {
    let t103166 = t103140 + t103142 - F::cast_from(0.23131639038696784278e-2_f64) * t95774 + F::cast_from(0.2601984143835408805e-1_f64) * t95779 - F::cast_from(0.48186823267806663678e-3_f64) * t95783 + F::cast_from(0.8673628188205199462e0_f64) * t99303 * t7420 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t7398 * t4423 * t231 - F::cast_from(0.34270468708064099208e-1_f64) * t95786 + F::cast_from(0.12851425765524037203e-1_f64) * t95790 + t103156 + F::cast_from(0.65049603595885220126e-3_f64) * t103158 + F::cast_from(0.11565819519348392139e-2_f64) * t103161 + F::cast_from(0.34270468708064099208e-2_f64) * t95794 + F::cast_from(0.19274729307122665471e-1_f64) * t95796 - F::cast_from(0.72280234901709995518e-2_f64) * t95798;
    t103166
}
