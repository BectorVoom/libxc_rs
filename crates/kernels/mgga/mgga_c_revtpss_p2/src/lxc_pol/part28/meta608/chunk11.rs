//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2118/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2118<F: Float>(t94484: F, t94485: F, t94498: F, t94501: F, t94503: F, t94505: F, t94509: F, t94511: F, t98236: F, t98239: F, t98244: F, t98245: F, t98253: F) -> F {
    let t98255 = -t98236 + t98239 + t94484 + F::new(7.0) / F::new(144.0) * t94485 + t98244 + F::cast_from(0.34299214494455789578e-2_f64) * t98245 + F::cast_from(0.54208002996571016774e-3_f64) * t94498 - F::cast_from(0.11433071498151929859e-3_f64) * t94501 + F::cast_from(0.20007875121765877254e-2_f64) * t94503 + F::cast_from(0.20007875121765877254e-2_f64) * t94505 + F::cast_from(0.50820002809285328226e-4_f64) * t94509 - F::cast_from(0.25410001404642664113e-4_f64) * t94511 - t98253 / F::new(48.0);
    t98255
}
