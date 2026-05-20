//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2140/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2140<F: Float>(t25317: F, t25383: F, t29644: F, t4533: F, t7070: F, t7769: F, t7770: F, t93378: F, t93382: F, t93384: F, t93391: F, t99303: F, t99485: F, t99487: F, t99491: F, t99493: F, t99496: F, t99502: F, t99520: F, t99522: F) -> F {
    let t106477 = -F::cast_from(0.34270468708064099208e-2_f64) * t93378 - t99485 - t99487 - F::cast_from(0.65049603595885220126e-3_f64) * t93382 - F::cast_from(0.96373646535613327357e-2_f64) * t93384 - t99491 + t99493 + F::cast_from(0.19274729307122665472e-1_f64) * t99496 - F::cast_from(0.26020884564615598386e1_f64) * t25383 * t29644 - t99502 - F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t7769 * t4533 - F::cast_from(0.13009920719177044025e-2_f64) * t99520 + F::cast_from(0.17347256376410398924e1_f64) * t99303 * t7770 - F::cast_from(0.34270468708064099208e-1_f64) * t99522 + F::cast_from(0.73171657588172351096e-2_f64) * t93391;
    t106477
}
