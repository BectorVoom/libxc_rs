//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 747/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk747<F: Float>(t45: F, t57: F, t2382: F, t606: F, t10326: F, t10356: F, t10457: F, t2258: F, t81: F, t10455: F, t150: F, t190: F, t80: F, t633: F, t766: F, t83: F, t637: F, t770: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t10460 = t2382 * t606;
    let t10466 = piecewise3(t155, 0.0, 8.0 / 27.0 * t10457 * t10356 + 4.0 / 3.0 * t10460 * t2258 - 4.0 / 3.0 * t81 * t10326);
    let t10467 = t10455 + t10466;
    let t10468 = t150 * t10467;
    let t10469 = t10468 * t190;
    let t10472 = t80 * t606;
    let t10478 = piecewise3(t151, 0.0, 8.0 / 27.0 * t633 * t10356 - 2.0 / 3.0 * t10472 * t2258 + 2.0 / 3.0 * t766 * t10326);
    let t10481 = t83 * t606;
    let t10487 = piecewise3(t155, 0.0, -8.0 / 27.0 * t637 * t10356 - 2.0 / 3.0 * t10481 * t2258 - 2.0 / 3.0 * t770 * t10326);
    (t10467, t10469, t10478, t10487)
}
