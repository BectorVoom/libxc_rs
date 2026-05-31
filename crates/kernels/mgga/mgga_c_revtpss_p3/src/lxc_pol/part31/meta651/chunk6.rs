//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2158/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2158<F: Float>(t27479: F, t4845: F, t100324: F, t100359: F, t100363: F, t100365: F, t100370: F, t100398: F, t1665: F, t19645: F, t19917: F, t25517: F, t25539: F, t6289: F, t6339: F, t7111: F, t93731: F) -> F {
    let t107188 = t27479 * t4845;
    let t107197 = F::cast_from(0.28582678745379824648e-3_f64) * t25517 * t19645 + t100359 + F::cast_from(0.45732285992607719437e-2_f64) * t100324 * t1665 - F::cast_from(0.57165357490759649296e-3_f64) * t107188 + F::cast_from(0.85748036236139473944e-3_f64) * t93731 * t6339 - t100363 - t100365 / F::cast_from(648.0_f64) + t100370 - t100398 + t7111 * t19917 / F::cast_from(288.0_f64) - t25539 * t6289 / F::cast_from(108.0_f64);
    t107197
}
