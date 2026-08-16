//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2195/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2195<F: Float>(t25411: F, t99495: F, t14495: F, t14979: F, t25391: F, t25392: F, t27189: F, t27265: F, t27353: F, t27357: F, t27358: F, t2772: F, t51608: F, t7053: F, t7070: F, t7071: F, t7073: F, t886: F, t92864: F, t93387: F, t93389: F, t93391: F, t99237: F, t99303: F, t99496: F, t99502: F, t99512: F, t99520: F) -> F {
    let t99522 = t25411 * t99495;
    let t99532 = F::cast_from(0.96373646535613327357e-2_f64) * t99496 - F::cast_from(0.17347256376410398924e1_f64) * t99237 * t27358 - t99502 + F::cast_from(0.17347256376410398924e1_f64) * t99303 * t7073 + F::cast_from(0.13170898365871023197e1_f64) * t27189 * t2772 - F::cast_from(0.65854491829355115987e0_f64) * t7053 * t14979 + F::cast_from(0.8673628188205199462e0_f64) * t27353 * t92864 * t14495 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t25392 * t99512 + F::cast_from(0.25702851531048074406e-1_f64) * t93387 - F::cast_from(0.14456046980341999104e-1_f64) * t93389 - F::cast_from(0.65049603595885220126e-3_f64) * t99520 - F::cast_from(0.17135234354032049604e-1_f64) * t99522 - F::cast_from(0.8673628188205199462e0_f64) * t27353 * t27357 * t51608 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t27265 * t886 + F::cast_from(0.14634331517634470219e-1_f64) * t93391;
    t99532
}
