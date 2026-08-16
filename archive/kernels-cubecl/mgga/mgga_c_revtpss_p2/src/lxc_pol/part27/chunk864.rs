//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 864/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk864<F: Float>(t2398: F, t2414: F, t10326: F, t190: F, t706: F, t2258: F, t750: F, t157: F, t36: F, t10356: F, t2401: F, t200: F, t45: F) -> (F, F, F, F, F, F) {
    let t10432 = F::cast_from(12.0_f64) * t2398 * t2414;
    let t10433 = t190 * t10326;
    let t10435 = F::cast_from(4.0_f64) * t706 * t10433;
    let t10436 = t750 * t2258;
    let t10437 = t706 * t10436;
    let t10438 = F::cast_from(12.0_f64) * t10437;
    let t10439 = t36 * t157;
    let t10440 = t190 * t10356;
    let t10442 = F::cast_from(24.0_f64) * t10439 * t10440;
    let t10443 = t2401 * t750;
    let t10444 = F::cast_from(3.0_f64) * t10443;
    let t10446 = F::cast_from(1.0_f64) / t200 / t45;
    (t10432, t10435, t10438, t10442, t10444, t10446)
}
